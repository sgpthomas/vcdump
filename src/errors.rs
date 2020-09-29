pub enum Error {
    JsonError(serde_json::error::Error),
    NoFile,
    IoError(std::io::Error),
    ChangeValue,
    ParseIntError(std::num::ParseIntError),
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Self {
        Error::JsonError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseIntError(err)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "{:?}", e.kind()),
            x => write!(f, "{:#?}", x),
        }
    }
}
