use common::context::Context;
use common::error::Error;
use entity::user::User;

/// UserService is the service for user management.
pub trait UserService {
    fn create_user(&self, ctx: Context, user: User) -> Result<(), Error>;

    fn delete_user(&self, ctx: Context, id: u64) -> Result<(), Error>;

    fn update_user(&self, ctx: Context, user: UserUpdate) -> Result<(), Error>;

    fn find_user_by_id(&self, ctx: Context, id: u64) -> Result<User, Error>;

    fn find_user_by_email(&self, ctx: Context, email: String) -> Result<User, Error>;

    fn find_users(&self, ctx: Context, filters: UserFilter) -> Result<(Vec<User>, usize), Error>;
}

/// UserUpdate is a struct for allowed fields to update a user.
#[derive(Clone, Debug, Default)]
pub struct UserUpdate {
    pub name: Option<String>,
}

// UserFilter is a struct for possibile filters for user search.
#[derive(Clone, Debug, Default)]
pub struct UserFilter {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub email: Option<String>,
}
