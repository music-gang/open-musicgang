use crate::Validable;
use chrono::prelude::*;
use openmusicgang_common::error::{Error, ErrorCode};

/// User is a struct to represent a user.
///
#[derive(Clone, Debug)]
pub struct User {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Validable for User {
    fn validate(&self) -> Result<(), Error> {
        if self.id == 0 {
            return Err(Error::new(
                ErrorCode::EINVALID,
                "id is required".to_string(),
            ));
        }

        if self.name == "" {
            return Err(Error::new(
                ErrorCode::EINVALID,
                "name is required".to_string(),
            ));
        }

        if self.email == "" {
            return Err(Error::new(
                ErrorCode::EINVALID,
                "email is required".to_string(),
            ));
        }

        if self.password == "" {
            return Err(Error::new(
                ErrorCode::EINVALID,
                "password is required".to_string(),
            ));
        }

        Ok(())
    }
}
