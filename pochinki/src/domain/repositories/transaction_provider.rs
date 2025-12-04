use anyhow::Result;
use async_trait::async_trait;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TransactionProvider {
    fn transaction<F, E, R>(&self, f: F) -> Result<R, E>
    where
        F: FnOnce(&mut PooledConnection<ConnectionManager<PgConnection>>) -> Result<R, E> + Send + 'static,
        E: From<diesel::result::Error> + From<anyhow::Error> + Send + 'static,
        R: Send + 'static;
}
