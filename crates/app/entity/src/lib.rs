use openmusicgang_common::error::Error;

pub mod user;

pub trait Validable {
    fn validate(&self) -> Result<(), Error>;
}
