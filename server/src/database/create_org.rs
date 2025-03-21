use super::{Pools, model::Service};
use metrics::counter;
use tokio::sync::oneshot;
use tokio_postgres::types::ToSql;

#[derive(Debug, Clone)]
pub struct CreateOrganisation {
    name: String,
    service: Service,
}

impl CreateOrganisation {
    pub fn new(name: &str, service: Service) -> Self {
        let name = name.to_lowercase();
        Self { name, service }
    }
}

pub(super) async fn create_organisation(
    pools: Pools,
    cmd: CreateOrganisation,
    reply: oneshot::Sender<Result<(), Error>>,
) {
    if reply.is_closed() {
        counter!("cov.database.actor.command_doa", "command" => "create-organisation").increment(1);
        return;
    }

    counter!("cov.database.command", "command" => "create-organisation").increment(1);
    let _ = reply.send(inner_create_organisation(pools, cmd).await);
}

async fn inner_create_organisation(pools: Pools, cmd: CreateOrganisation) -> Result<(), Error> {
    let conn = pools.read_write().await?;

    // Pipeline the check and insert queries at once.
    let args: &[&(dyn ToSql + Sync)] = &[&cmd.service, &cmd.name];
    let existing = conn.query_opt(
        "SELECT 1 FROM organisation WHERE service = $1 AND name = $2",
        args,
    );

    let insert = conn.prepare("INSERT INTO organisation (service, name) VALUES ($1, $2)");
    let (existing, insert) = tokio::try_join!(existing, insert)?;
    if existing.is_some() {
        return Err(Error::AlreadyExists);
    }

    let _ = conn.execute(&insert, args).await?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to acquire connection from pool")]
    Pool(#[from] deadpool_postgres::PoolError),

    #[error("failed to query/update database")]
    Postgres(#[from] tokio_postgres::Error),

    #[error("the reply channel was dropped without a reply. this is a bug")]
    ReplyClosed,

    #[error("the organisation already exists in the database")]
    AlreadyExists,
}
