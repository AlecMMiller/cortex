#[cfg(test)]
pub mod test_util {
    use rusqlite::Connection;

    use crate::database::migration::migrate;

    pub fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();

        migrate(&conn).unwrap();

        conn
    }
}
