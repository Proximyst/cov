use sqlx::migrate::Migrator;

pub const MIGRATOR: Migrator = sqlx::migrate!("./src/migrations");
