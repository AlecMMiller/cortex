mod commands;
pub mod database;
mod macros;
pub mod models;
mod setup;

use database::migration::migrate;
use rusqlite::Transaction;

pub fn migrate_db(tx: &Transaction) {
    migrate(&tx).unwrap();
}

pub fn test_fn() {}
