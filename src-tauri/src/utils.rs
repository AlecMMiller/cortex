use diesel::{
    r2d2::{self, ConnectionManager},
    SqliteConnection,
};

use crate::SqlitePool;

pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn get_connection(pool: SqlitePool) -> PooledConnection {
    // TODO consider handling this error instead of panicking
    let conn = pool
        .get()
        .expect("Could not establish connection to database");
    conn
}
