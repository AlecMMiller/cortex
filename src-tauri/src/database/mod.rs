use rusqlite::Transaction;

pub mod attribute;
pub mod attribute_schema;
pub mod attribute_type;
pub mod entity;
pub mod entity_schema;
#[cfg(test)]
mod entity_test;
pub mod migration;
mod response_map;
mod test;

pub trait New<T> {
    fn new(tx: &Transaction, data: T) -> rusqlite::Result<Self>
    where
        Self: Sized;
}

pub trait Get<T> {
    fn get(tx: &Transaction, id: &T) -> rusqlite::Result<Self>
    where
        Self: Sized;
}

pub trait GetMany<T> {
    fn get_many(tx: &Transaction, id: &T) -> rusqlite::Result<Vec<Self>>
    where
        Self: Sized;
}

pub trait Insert<T, V: ?Sized> {
    fn insert(&self, tx: &Transaction, target: &T, val: &V) -> rusqlite::Result<()>;
}

pub trait Delete {
    fn delete(self, tx: &Transaction) -> rusqlite::Result<()>;
}

pub trait SetValue<T> {
    fn set(&self, tx: &Transaction, value: T) -> rusqlite::Result<()>;
}
