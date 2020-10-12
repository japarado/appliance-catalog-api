use crate::errors::ServiceError;

pub type Multiple<T> = Result<Vec<T>, ServiceError>;
pub type Single<T> = Result<T, ServiceError>;

pub mod user;
