use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteId(String);

impl NoteId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl Into<String> for NoteId {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timestamp(i64);

impl From<i64> for Timestamp {
    fn from(ts: i64) -> Self {
        Self(ts)
    }
}

impl From<Timestamp> for i64 {
    fn from(ts: Timestamp) -> Self {
        ts.0
    }
}

impl Timestamp {
    pub fn now() -> Self {
        Self(chrono::Utc::now().timestamp())
    }
}

impl From<String> for NoteId {
    fn from(uuid: String) -> Self {
        Self(uuid)
    }
}

impl NoteId {
    pub fn to_string(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub uuid: NoteId,
    pub title: String,
    pub body: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
