use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use diesel_async::{pooled_connection::bb8, AsyncPgConnection};

use alw_error::Error;

pub type PooledConnection = bb8::PooledConnection<'static, AsyncPgConnection>;
pub type Pool = bb8::Pool<AsyncPgConnection>;

pub struct DatabaseConnection(pub PooledConnection);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    Pool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);
        //let conn = pool.get().await?;
        let conn = pool.get_owned().await?;
        Ok(DatabaseConnection(conn))
    }
}
