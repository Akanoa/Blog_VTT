use diesel::r2d2::PoolError;
use pwhash::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum MyError {
    Password(Error),
    Pool(PoolError),
    Diesel(diesel::result::Error),
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for MyError {}

impl From<PoolError> for MyError {
    fn from(value: PoolError) -> Self {
        MyError::Pool(value)
    }
}

impl From<pwhash::error::Error> for MyError {
    fn from(value: Error) -> Self {
        MyError::Password(value)
    }
}

impl From<diesel::result::Error> for MyError {
    fn from(value: diesel::result::Error) -> Self {
        MyError::Diesel(value)
    }
}
