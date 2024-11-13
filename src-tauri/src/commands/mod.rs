pub mod notes;
pub mod settings;
pub mod tags;

use specta::Type;
use tantivy::TantivyError;

#[derive(Debug, Type, thiserror::Error)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    #[error("io error: {0}")]
    Tantivy(
        #[serde(skip)]
        #[from]
        TantivyError,
    ),
    #[error("foo")]
    Diesel(
        #[serde(skip)]
        #[from]
        diesel::result::Error,
    ),
    #[error("bar")]
    Serde(
        #[serde(skip)]
        #[from]
        serde_json::Error,
    ),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
