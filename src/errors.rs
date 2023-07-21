use std::fmt;
#[derive(Debug)]
#[allow(dead_code)]
pub enum ServiceError {
    FailedToCreateDB,
    FailedToFoundCollection,
    FailedToParseCommand,
    WrongNumberOfArgs,
    InvalidArgument
}
impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::FailedToCreateDB => write!(f, "Failed to start database"),
            ServiceError::FailedToFoundCollection => write!(f, "Failed to find collection"),
            ServiceError::FailedToParseCommand => write!(f, "Failed to parse command"),
            ServiceError::WrongNumberOfArgs => write!(f, "Wrong number of arguments"),
            ServiceError::InvalidArgument => write!(f, "Invalid argument"),
        }
    }
}