use std::fmt;

#[derive(Debug)]
pub struct Error {
    kind: Kind,
    message: String,
}

impl Error {
    pub fn new_connection_error(message: String) -> Error {
        Error { 
            kind: Kind::ConnectionError, 
            message 
        }
    }

    pub fn new_repository_error(message: String) -> Error {
        Error { 
            kind: Kind::RepositoryError, 
            message 
        }
    }

    pub fn new_kana_error(message: String) -> Error {
        Error {
            kind: Kind::KanaError,
            message
        }
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }
}

#[derive(Debug)]
pub enum Kind {
    ConnectionError,
    RepositoryError,
    KanaError,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            Kind::ConnectionError => "Error connecting to database",
            Kind::RepositoryError => "Error querying database", 
            Kind::KanaError => "Error transforming kana",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            Kind::ConnectionError => write!(f, "{}", self.message()),
            Kind::RepositoryError => write!(f, "{}", self.message()),
            Kind::KanaError => write!(f, "{}", self.message()),
        }   
    }
}