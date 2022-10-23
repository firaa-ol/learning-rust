pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Initialization of database failed : {0}")]
    DbInitFailed(std::io::Error),

    #[error("serde_json error: {0}")]
    SerdeJsonError(String),

    #[error("IO error: {0}")]
    Io(std::io::ErrorKind),

    #[error("'{0}' is not present in the list")]
    ToDoItemNotFound(String),

    #[error("No Todo Items")]
    EmptyToDos,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.kind())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJsonError(err.to_string())
    }
}
