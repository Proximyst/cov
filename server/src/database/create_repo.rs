use super::{Pools, model::Service};
use metrics::counter;
use tokio::sync::oneshot;
use tokio_postgres::types::ToSql;

#[derive(Debug, Clone)]
pub struct CreateRepository {
    service: Service,
    org_name: String,
    name: String,
}

impl CreateRepository {
    pub fn new(service: Service, org_name: &str, name: &str) -> Self {
        let org_name = org_name.to_lowercase();
        let name = name.to_lowercase();
        Self {
            service,
            org_name,
            name,
        }
    }
}

pub(super) async fn create_repository(
    pools: Pools,
    cmd: CreateRepository,
    reply: oneshot::Sender<Result<(), Error>>,
) {
    if reply.is_closed() {
        counter!("cov.database.actor.command_doa", "command" => "create-repository").increment(1);
        return;
    }

    counter!("cov.database.command", "command" => "create-repository").increment(1);
    let _ = reply.send(inner_create_repo(pools, cmd).await);
}

async fn inner_create_repo(pools: Pools, cmd: CreateRepository) -> Result<(), Error> {
    let conn = pools.read_write().await?;

    // Pipeline the check and insert queries at once.
    let args: &[&(dyn ToSql + Sync)] = &[&cmd.service, &cmd.org_name];
    let org = conn.query_opt(
        "SELECT id FROM organisation WHERE service = $1 AND name = $2",
        args,
    );

    let args: &[&(dyn ToSql + Sync)] = &[&cmd.service, &cmd.org_name, &cmd.name];
    let existing_repo = conn.query_opt(
        "SELECT 1 FROM repository r JOIN organisation o ON r.organisation_id = o.id WHERE o.service = $1 AND o.name = $2 AND r.name = $3",
        args,
    );

    let insert = conn.prepare("INSERT INTO repository (organisation_id, name) VALUES ((SELECT id FROM organisation WHERE service = $1 AND name = $2), $3)");
    let (org, existing_repo, insert) = tokio::try_join!(org, existing_repo, insert)?;
    if org.is_none() {
        return Err(Error::OrgNotFound);
    }
    if existing_repo.is_some() {
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

    #[error("the organisation given does not exist")]
    OrgNotFound,

    #[error("the repository already exists in the database")]
    AlreadyExists,
}
