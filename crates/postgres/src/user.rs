use std::sync::{Arc, Mutex};

use chrono::prelude::*;

use openmusicgang_app::context::{AppContext, Context};
use openmusicgang_entity::user::User;
use openmusicgang_entity::Validable;
use openmusicgang_err::error::{Error, ErrorCode};
use openmusicgang_service::user_service::{
    UserFilter, UserService as UserServiceTrait, UserUpdate,
};
use postgres::types::ToSql;
use postgres::Transaction;

use crate::db::DB;
use crate::{
    delete_user_params, delete_user_sql, format_limit_offset, insert_user_params, insert_user_sql,
    select_users_sql, where_condition_eq,
};

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

    /// Deletes a user.
    fn delete_user(&self, _ctx: AppContext, _id: i64) -> Result<(), Error> {
        let mut mutex_db = self.db.lock().map_err(|_| {
            Error::new(
                ErrorCode::EINTERNAL,
                "Could not acquire lock on database".to_string(),
            )
        })?;

        let mut tx = mutex_db.begin_tx()?;

        delete_user(_ctx, &mut tx, _id)?;

        tx.commit().map_err(|_| {
            Error::new(
                ErrorCode::EINTERNAL,
                "Could not commit transaction to database".to_string(),
            )
        })?;

        Ok(())
    }

    fn update_user(&self, _ctx: AppContext, _user: UserUpdate) -> Result<(), Error> {
        todo!()
    }

    /// Get a user by id.
    fn find_user_by_id(&self, ctx: AppContext, id: i64) -> Result<User, Error> {
        let mut mutex_db = self.db.lock().map_err(|_| {
            Error::new(
                ErrorCode::EINTERNAL,
                "Could not acquire lock on database".to_string(),
            )
        })?;

        let mut tx = mutex_db.begin_tx()?;

        let user = find_user_by_id(ctx, &mut tx, id)?;

        Ok(user)
    }

    /// Get a user by email.
    fn find_user_by_email(&self, ctx: AppContext, email: String) -> Result<User, Error> {
        let mut mutex_db = self.db.lock().map_err(|_| {
            Error::new(
                ErrorCode::EINTERNAL,
                "Could not acquire lock on database".to_string(),
            )
        })?;

        let mut tx = mutex_db.begin_tx()?;

        let user = find_user_by_email(ctx, &mut tx, email)?;

        Ok(user)
    }

    /// Returns a vector of users based on passed filters, also returns the total number of users.
    fn find_users(&self, ctx: AppContext, filters: UserFilter) -> Result<(Vec<User>, i64), Error> {
        let mut mutex_db = self.db.lock().map_err(|_| {
            Error::new(
                ErrorCode::EINTERNAL,
                "Could not acquire lock on database".to_string(),
            )
        })?;

        let mut tx = mutex_db.begin_tx()?;

        find_users(ctx, &mut tx, filters)
    }
}

/// create_user inserts a new user into the database.
/// Handles the create_user Business Logic.
fn create_user(_ctx: AppContext, tx: &mut Transaction, user: &mut User) -> Result<(), Error> {
    user.created_at = Utc::now();
    user.updated_at = Utc::now();

    user.validate()?;

    let row = tx
        .query_one(insert_user_sql!().as_str(), insert_user_params!(user))
        .map_err(|error| Error::new(ErrorCode::EINTERNAL, error.to_string()))?;

    user.id = row.get(0);

    Ok(())
}

/// delete_user deletes a user from the database.
/// Handles the delete_user Business Logic.
/// Returns EUNAUTHORIZED if the user is not authorized to delete the user.
fn delete_user(ctx: AppContext, tx: &mut Transaction, id: i64) -> Result<(), Error> {
    let user = find_user_by_id(ctx.clone(), tx, id)?;

    if let Some(id) = Context::user_id_from_context(ctx) {
        if id != user.id {
            return Err(Error::new(
                ErrorCode::EUNAUTHORIZED,
                "You do not have permission to delete this user".to_string(),
            ));
        }
    }

    tx.execute(delete_user_sql!(), delete_user_params!(id))
        .map_err(|error| Error::new(ErrorCode::EINTERNAL, error.to_string()))?;

    Ok(())
}

/// find_user_by_email finds a user by email.
/// Handles the find_user_by_email Business Logic.
/// Returns ENOTFOUND if the user is not found.
fn find_user_by_email(ctx: AppContext, tx: &mut Transaction, email: String) -> Result<User, Error> {
    let mut filters = UserFilter::default();
    filters.email = Some(email);

    let result = find_users(ctx, tx, filters)?;

    if result.1 == 0 {
        return Err(Error::new(
            ErrorCode::ENOTFOUND,
            "User not found".to_string(),
        ));
    }

    Ok(result.0[0].clone())
}

/// find_user_by_id returns a user by id.
/// Returns ENOTFOUND if the user does not exist.
/// Handles the find_user_by_id Business Logic.
fn find_user_by_id(ctx: AppContext, tx: &mut Transaction, id: i64) -> Result<User, Error> {
    let mut filters = UserFilter::default();
    filters.id = Some(id);

    let result = find_users(ctx, tx, filters)?;

    if result.1 == 0 {
        return Err(Error::new(
            ErrorCode::ENOTFOUND,
            "User not found".to_string(),
        ));
    }

    Ok(result.0[0].clone())
}

/// find_users finds users in the database based on the filters.
/// Handles the find_users Business Logic.
fn find_users(
    _ctx: AppContext,
    tx: &mut Transaction,
    filters: UserFilter,
) -> Result<(Vec<User>, i64), Error> {
    let mut where_conditions = vec!["1 = 1".to_string()];
    let mut args: Vec<&(dyn ToSql + Sync)> = vec![];
    let mut args_counter = 1;

    if filters.id.is_some() {
        where_conditions.push(where_condition_eq!("id", args_counter));
        args_counter += 1;
        args.push(&filters.id);
    }

    if filters.name.is_some() {
        where_conditions.push(where_condition_eq!("name", args_counter));
        args_counter += 1;
        args.push(&filters.name);
    }

    if filters.email.is_some() {
        where_conditions.push(where_condition_eq!("email", args_counter));
        args_counter += 1;
        drop(args_counter);
        args.push(&filters.email);
    }

    let query = select_users_sql!(
        where_conditions,
        format_limit_offset!(filters.limit, filters.offset)
    );

    let rows = tx
        .query(query.as_str(), &args)
        .map_err(|error| Error::new(ErrorCode::EINTERNAL, error.to_string()))?;

    let mut users: Vec<User> = vec![];
    let mut tot_results = 0;

    for row in rows {
        let mut user = User::new();

        user.id = row.get(0);
        user.name = row.get(1);
        user.email = row.get(2);
        user.password = row.get(3);
        user.created_at = row.get(4);
        user.updated_at = row.get(5);
        tot_results = row.get(6);

        users.push(user);
    }

    return Ok((users, tot_results));
}

#[cfg(test)]
mod tests {

    use openmusicgang_app::context::Context;

    use crate::test_utils::{must_open_db, must_truncate_table};

    use super::*;

    #[test]
    fn test_user_service() {
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

        let res = user_service.find_user_by_id(Context::background(), 1);
        assert!(res.is_ok());
        let user = res.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Bob Smith");
        assert_eq!(user.email, "bob.smith@test.com");
        assert_eq!(user.password, Some("password".to_string()));

        let res = user_service
            .find_user_by_email(Context::background(), "bob.smith@test.com".to_string());
        assert!(res.is_ok());
    }
}
