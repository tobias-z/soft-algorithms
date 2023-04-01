use async_trait::async_trait;
use sqlx::{Database, Error, Pool, Postgres};

#[async_trait]
pub trait Connection<T: Database> {
    async fn connect(&self) -> Result<Pool<T>, Error>;
}

pub struct PostgresConnection {
    max_connections: u32,
}

impl PostgresConnection {
    pub fn new(max_connections: u32) -> Self {
        Self { max_connections }
    }
}

#[async_trait]
impl Connection<Postgres> for PostgresConnection {
    async fn connect(&self) -> Result<Pool<Postgres>, Error> {
        let username = std::env::var("POSTGRES_USER").expect("variable POSTGRES_USER was missing");
        let password =
            std::env::var("POSTGRES_PASSWORD").expect("variable POSTGRES_PASSWORD was missing");
        let host = std::env::var("POSTGRES_HOST").expect("variable POSTGRES_HOST was missing");
        let database =
            std::env::var("POSTGRES_DATABASE").expect("variable POSTGRES_DATABASE was missing");
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(self.max_connections)
            .connect(&format!(
                "postgres://{}:{}@{}/{}",
                username, password, host, database
            ))
            .await
    }
}
