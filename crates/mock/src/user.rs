use openmusicgang_app::context::AppContext;
use openmusicgang_entity::user::User;
use openmusicgang_err::error::Error;
use openmusicgang_service::user_service::{
    UserFilter, UserService as UserServiceTrait, UserUpdate,
};

pub struct UserService {
    pub create_user_fn: Option<fn(AppContext, &mut User) -> Result<(), Error>>,
    pub delete_user_fn: Option<fn(AppContext, i64) -> Result<(), Error>>,
    pub update_user_fn: Option<fn(AppContext, i64, UserUpdate) -> Result<User, Error>>,
    pub find_user_by_id_fn: Option<fn(AppContext, i64) -> Result<User, Error>>,
    pub find_user_by_email_fn: Option<fn(AppContext, String) -> Result<User, Error>>,
    pub find_users_fn: Option<fn(AppContext, UserFilter) -> Result<(Vec<User>, i64), Error>>,
}

impl UserServiceTrait for UserService {
    fn create_user(&self, ctx: AppContext, user: &mut User) -> Result<(), Error> {
        if let Some(f) = self.create_user_fn {
            return f(ctx, user);
        }
        panic!("create_user_fn not set");
    }

    fn delete_user(&self, ctx: AppContext, id: i64) -> Result<(), Error> {
        if let Some(f) = self.delete_user_fn {
            return f(ctx, id);
        }
        panic!("delete_user_fn not set");
    }

    fn update_user(&self, ctx: AppContext, id: i64, user: UserUpdate) -> Result<User, Error> {
        if let Some(f) = self.update_user_fn {
            return f(ctx, id, user);
        }
        panic!("update_user_fn not set");
    }

    fn find_user_by_id(&self, ctx: AppContext, id: i64) -> Result<User, Error> {
        if let Some(f) = self.find_user_by_id_fn {
            return f(ctx, id);
        }
        panic!("find_user_by_id_fn not set");
    }

    fn find_user_by_email(&self, ctx: AppContext, email: String) -> Result<User, Error> {
        if let Some(f) = self.find_user_by_email_fn {
            return f(ctx, email);
        }
        panic!("find_user_by_email_fn not set");
    }

    fn find_users(&self, ctx: AppContext, filters: UserFilter) -> Result<(Vec<User>, i64), Error> {
        if let Some(f) = self.find_users_fn {
            return f(ctx, filters);
        }
        panic!("find_users_fn not set");
    }
}
