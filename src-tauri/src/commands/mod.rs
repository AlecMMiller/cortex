pub mod notes;
pub mod settings;

use tantivy::TantivyError;

use crate::models::notes::Error as NoteError;

#[derive(Debug)]
pub enum Error {
    Tantivy(TantivyError),
    Diesel(diesel::result::Error),
    Serde(serde_json::Error),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Self::Tantivy(err) => err.to_string(),
            Self::Diesel(err) => err.to_string(),
            Self::Serde(err) => err.to_string(),
        }
    }
}

impl From<NoteError> for Error {
    fn from(err: NoteError) -> Self {
        match err {
            NoteError::Tantivy(err) => Self::Tantivy(err),
            NoteError::Diesel(err) => Self::Diesel(err),
            NoteError::Serde(err) => Self::Serde(err),
        }
    }
}

impl From<TantivyError> for Error {
    fn from(err: TantivyError) -> Self {
        Self::Tantivy(err)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Self::Diesel(err)
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
