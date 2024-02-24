use diesel_async::{pooled_connection::deadpool, AsyncPgConnection};


pub type DatabaseConnection = AsyncPgConnection;
pub type Pool = deadpool::Pool<DatabaseConnection>;

// pub struct DatabaseConnection(pub Connection);
