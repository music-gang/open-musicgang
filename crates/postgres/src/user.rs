use std::sync::{Arc, Mutex};

use chrono::prelude::*;

use openmusicgang_app::context::AppContext;
use openmusicgang_app::error::{Error, ErrorCode};
use openmusicgang_entity::user::User;
use openmusicgang_entity::Validable;
use openmusicgang_service::user_service::{
    UserFilter, UserService as UserServiceTrait, UserUpdate,
};
use postgres::Transaction;

use crate::db::DB;
use crate::{insert_user_params, insert_user_sql};

/// UserService is a struct that implements the UserServiceTrait for the postgres crate.
pub struct UserService {
    db: Arc<Mutex<DB>>,
}

impl UserService {
    /// Create a new UserService struct
    pub fn new(db: Arc<Mutex<DB>>) -> UserService {
        UserService { db }
    }
}

impl UserServiceTrait for UserService {
    /// Create a new user.
    fn create_user(&self, _ctx: AppContext, user: &mut User) -> Result<(), Error> {
        let mut mutex_db = self.db.lock().map_err(|_| {
            Error::new(
                ErrorCode::EINTERNAL,
                "Could not acquire lock on database".to_string(),
            )
        })?;

        let mut tx = mutex_db.begin_tx()?;

        create_user(_ctx, &mut tx, user)?;

        tx.commit().map_err(|_| {
            Error::new(
                ErrorCode::EINTERNAL,
                "Could not commit transaction to database".to_string(),
            )
        })?;

        Ok(())
    }

    fn delete_user(&self, _ctx: AppContext, _id: u64) -> Result<(), Error> {
        todo!()
    }

    fn update_user(&self, _ctx: AppContext, _user: UserUpdate) -> Result<(), Error> {
        todo!()
    }

    fn find_user_by_id(&self, _ctx: AppContext, _id: u64) -> Result<User, Error> {
        todo!()
    }

    fn find_user_by_email(&self, _ctx: AppContext, _email: String) -> Result<User, Error> {
        todo!()
    }

    fn find_users(
        &self,
        _ctx: AppContext,
        _filters: UserFilter,
    ) -> Result<(Vec<User>, usize), Error> {
        todo!()
    }
}

/// create_user inserts a new user into the database.
/// Handles the create_user Business Logic.
fn create_user(_ctx: AppContext, tx: &mut Transaction, user: &mut User) -> Result<(), Error> {
    user.created_at = Utc::now();
    user.updated_at = Utc::now();

    user.validate()?;

    let row = tx
        .query_one(insert_user_sql!(), insert_user_params!(user))
        .map_err(|error| Error::new(ErrorCode::EINTERNAL, error.to_string()))?;

    user.id = row.get(0);

    Ok(())
}

#[cfg(test)]
mod tests {

    use openmusicgang_app::context::Context;

    use crate::test_utils::{must_open_db, must_truncate_table};

    use super::*;

    #[test]
    fn create_user() {
        let mut db = must_open_db();

        must_truncate_table(&mut db, "users");

        let user_service = UserService::new(Arc::new(Mutex::new(db)));

        let mut user = User::new();

        user.name = "Bob Smith".to_string();
        user.email = "bob.smith@test.com".to_string();
        user.password = Some("password".to_string());

        let res = user_service.create_user(Context::background(), &mut user);
        if let Err(error) = res {
            panic!("{}", error);
        }

        let res = user_service.create_user(Context::background(), &mut user);
        assert!(res.is_err());

        assert_eq!(user.id, 1);
    }
}
