#[derive(Debug)]
pub enum BeanstalkdError { ConnectionError, RequestError }

pub type BeanstalkdResult<T> = Result<T, BeanstalkdError>;
