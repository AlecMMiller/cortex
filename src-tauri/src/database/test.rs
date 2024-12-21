#[cfg(test)]
pub mod test_util {
    use rusqlite::Connection;

    use crate::database::migration::migrate;

    pub fn setup() -> Connection {
        let mut conn = Connection::open_in_memory().unwrap();

        let tx = conn.transaction().unwrap();
        migrate(&tx).unwrap();
        tx.commit().unwrap();

        conn
    }
}
