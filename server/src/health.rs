use chrono::{DateTime, Utc};
use metrics::{counter, gauge};
use proto::health::{Health, State};
use tokio::{
    sync::{mpsc, watch},
    task::JoinSet,
};
use tracing::{error, trace};

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! components {
    ($ty:ident: $($name:ident),+ $(,)*) => {
        /// An individual component in the system.
        #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
        pub enum $ty {
            $($name),*
        }

        impl $ty {
            pub const COUNT: usize = count!($($name)*);

            pub fn name(&self) -> &'static str {
                match self {
                    $(Self::$name => stringify!($name)),*
                }
            }

            pub fn all_with(now: DateTime<Utc>, state: State) -> Health {
                use std::collections::HashMap;
                let mut components = HashMap::with_capacity(Self::COUNT);
                $(components.insert(stringify!($name).to_string(), state.clone());)*
                Health { last_update: now, components }
            }

            pub fn all_unknown(now: DateTime<Utc>) -> Health {
                Self::all_with(now, State::Unknown)
            }
        }
    };
}

// perf: we could use a fixed_map here to avoid the hashmap allocation.
components![ Component:
    Database,
    RestApiActor,
    HealthApiActor,
];

pub fn spawn_tracking_actor(
    set: &mut JoinSet<()>,
) -> (mpsc::Sender<(Component, State)>, watch::Receiver<Health>) {
    let (comp_tx, comp_rx) = mpsc::channel(1);
    let (health_tx, health_rx) = watch::channel(Component::all_unknown(Utc::now()));
    set.spawn(act(comp_rx, health_tx));
    (comp_tx, health_rx)
}

async fn act(mut rx: mpsc::Receiver<(Component, State)>, tx: watch::Sender<Health>) {
    let mut health = tx.borrow().clone();
    mark_health(health.clone());

    while let Some((component, state)) = rx.recv().await {
        counter!("cov.health.updates", "component" => component.name()).increment(1);
        trace!(?component, ?state, "got component health update");
        health.last_update = Utc::now();
        health
            .components
            .insert(component.name().to_string(), state);
        if let Err(watch::error::SendError(_)) = tx.send(health.clone()) {
            error!("health actor has no receiver anymore. shutting it down");
            break;
        }

        mark_health(health.clone());
    }
}

fn mark_health(h: Health) {
    let mut healthy = true;
    for (comp, state) in h.components {
        let ok = matches!(state, State::Healthy);
        healthy = healthy && ok;
        gauge!("cov.health.component", "component" => comp).set(if ok { 1 } else { 0 });
    }

    gauge!("cov.health.healthy").set(if healthy { 1 } else { 0 });
}
