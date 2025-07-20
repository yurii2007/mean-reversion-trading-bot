use std::fmt;

#[derive(Debug)]
pub enum ClientError {
    ConnectionError(String),
    InternalCommunicationError,
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            self::ClientError::ConnectionError(reason) => f.write_str(reason),
            self::ClientError::InternalCommunicationError => {
                f.write_str("Failed to communicate between clients")
            }
        }
    }
}
