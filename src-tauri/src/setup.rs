use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub type SqlitePool = Pool<SqliteConnectionManager>;

pub struct PoolWrapper {
    pub pool: SqlitePool,
}
