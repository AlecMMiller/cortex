pub mod database;
mod macros;
pub mod models;
pub mod setup;
mod utils;

use database::migration::migrate;
use rusqlite::Transaction;

pub fn migrate_db(tx: &Transaction) {
    migrate(&tx).unwrap();
}

pub fn test_fn() {}
