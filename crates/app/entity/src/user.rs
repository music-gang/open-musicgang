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
    pub password: Option<String>,
}

impl User {
    pub fn new() -> User {
        User {
            id: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name: "".to_string(),
            email: "".to_string(),
            password: None,
        }
    }
}

impl Validable for User {
    fn validate(&self) -> Result<(), Error> {
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

        if self.password.is_some() && self.password.as_ref().unwrap().len() == 0 {
            return Err(Error::new(
                ErrorCode::EINVALID,
                "password cannot be empty if provided".to_string(),
            ));
        }

        Ok(())
    }
}
