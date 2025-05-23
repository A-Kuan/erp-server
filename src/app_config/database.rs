use std::future::Future;
use std::pin::Pin;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgConnection, PgPool, Pool, Postgres};

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

pub trait TransactionExt {
    async fn with_transaction<F, T, E>(&self, operation: F) -> Result<T, E>
    where
        F: FnOnce(&mut PgConnection) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + '_>>,
        E: From<sqlx::Error>;
}

impl TransactionExt for PgPool {

    async fn with_transaction<F, T, E>(&self, operation: F) -> Result<T, E>
    where
        F: FnOnce(&mut PgConnection) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + '_>>,
        E: From<sqlx::Error>,
    {
        let mut tx = self.begin().await?;
        let result = operation(&mut tx).await;

        match result {
            Ok(output) => {
                tx.commit().await?;
                Ok(output)
            },
            Err(e) => {
                let _ = tx.rollback().await;
                Err(e)
            }
        }
    }
}