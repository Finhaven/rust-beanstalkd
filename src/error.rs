use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum BeanstalkdError {
    ConnectionError,
    UnknownStatusError(String),
    RequestError,
}

impl Error for BeanstalkdError {
    fn description(&self) -> &str {
        match self {
            BeanstalkdError::ConnectionError => "Connection error occurred",
            BeanstalkdError::RequestError => "Request error occurred",
            BeanstalkdError::UnknownStatusError(_) => "Unknown status",
        }
    }
}

impl Display for BeanstalkdError {
    fn fmt(&self, formatter: &mut Formatter) -> ::std::fmt::Result {
        let message = match self {
            BeanstalkdError::ConnectionError => "Connection error occurred".to_string(),
            BeanstalkdError::RequestError => "Request error occurred".to_string(),
            BeanstalkdError::UnknownStatusError(status) => format!("Unknown status: {}", status),
        };
        message.fmt(formatter)
    }
}

pub type BeanstalkdResult<T> = Result<T, BeanstalkdError>;
