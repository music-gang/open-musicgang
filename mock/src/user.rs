use common::context::Context;
use common::error::Error;
use entity::user::User;
use service::user_service::{UserFilter, UserUpdate};

pub struct UserService {
    pub create_user_fn: Option<fn(Context, User) -> Result<(), Error>>,
    pub delete_user_fn: Option<fn(Context, u64) -> Result<(), Error>>,
    pub update_user_fn: Option<fn(Context, UserUpdate) -> Result<(), Error>>,
    pub find_user_by_id_fn: Option<fn(Context, u64) -> Result<User, Error>>,
    pub find_user_by_email_fn: Option<fn(Context, String) -> Result<User, Error>>,
    pub find_users_fn: Option<fn(Context, UserFilter) -> Result<(Vec<User>, usize), Error>>,
}

impl service::user_service::UserService for UserService {
    fn create_user(&self, ctx: Context, user: User) -> Result<(), Error> {
        if let Some(f) = self.create_user_fn {
            return f(ctx, user);
        }
        panic!("create_user_fn not set");
    }

    fn delete_user(&self, ctx: Context, id: u64) -> Result<(), Error> {
        if let Some(f) = self.delete_user_fn {
            return f(ctx, id);
        }
        panic!("delete_user_fn not set");
    }

    fn update_user(&self, ctx: Context, user: UserUpdate) -> Result<(), Error> {
        if let Some(f) = self.update_user_fn {
            return f(ctx, user);
        }
        panic!("update_user_fn not set");
    }

    fn find_user_by_id(&self, ctx: Context, id: u64) -> Result<User, Error> {
        if let Some(f) = self.find_user_by_id_fn {
            return f(ctx, id);
        }
        panic!("find_user_by_id_fn not set");
    }

    fn find_user_by_email(&self, ctx: Context, email: String) -> Result<User, Error> {
        if let Some(f) = self.find_user_by_email_fn {
            return f(ctx, email);
        }
        panic!("find_user_by_email_fn not set");
    }

    fn find_users(
        &self,
        ctx: Context,
        filters: service::user_service::UserFilter,
    ) -> Result<(Vec<User>, usize), Error> {
        if let Some(f) = self.find_users_fn {
            return f(ctx, filters);
        }
        panic!("find_users_fn not set");
    }
}
