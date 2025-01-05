use std::time::Duration;

use derive_more::Deref;
use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DBConfig {
    pub url: String,
    pub min_connections: Option<u32>,
    pub max_connections: usize,
    pub connect_timeout: u64,
    pub idle_timeout: Option<u64>,
}

#[derive(Clone, Debug, Deref)]
pub struct DB {
    pub conn: DatabaseConnection,
}

impl DB {
    pub async fn connect(config: &DBConfig) -> Result<Self, DbErr> {
        let mut options: ConnectOptions = config.url.clone().into();

        let max_connections = u32::try_from(config.max_connections)
            .map_err(|_| DbErr::Custom("Max connections value out of range".to_string()))?;

        options
            .max_connections(max_connections)
            .min_connections(config.min_connections.unwrap_or_default())
            .connect_timeout(Duration::from_secs(config.connect_timeout));

        tracing::info!("Connecting to database: {:?}", config);

        if let Some(idle_timeout) = config.idle_timeout {
            options.idle_timeout(Duration::from_secs(idle_timeout));
        }
        let conn = Database::connect(options).await?;

        Ok(Self { conn })
    }

    pub async fn run_migrations(&self) -> std::result::Result<(), DbErr> {
        migration::Migrator::up(self.conn(), None).await?;

        Ok(())
    }

    pub fn conn(&self) -> &DatabaseConnection {
        &self.conn
    }
}
