use crate::models::Error;

pub type CanisterResult<T> = Result<T, Error>;
