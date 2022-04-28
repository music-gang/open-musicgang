use common::error::Error;

pub mod user;

pub trait Validable {
    fn validate(&self) -> Option<Error>;
}
