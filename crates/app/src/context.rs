use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use openmusicgang_entity::user::User;

/// Value is an enum to represent all possibile values accepted inside the context.
/// You can define your own values here.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    Number(f64),
    Bool(bool),
    User(User),
    Null,
}

static CONTEXT_KEY_USER: &str = "user";

/// AppContext is an alias for Thread-Safe Context.
pub type AppContext = Arc<Mutex<Context>>;

/// Context is a struct to represent the context of the application.
pub struct Context {
    /// parent context of current context, if first level context, parent is None.
    parent_ctx: Option<AppContext>,
    /// hashmap of values in the context.
    values: HashMap<String, Value>,
}

impl Context {
    /// Returns an empty context.
    /// You should use this when you don't know which context to use.
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn TODO() -> AppContext {
        Arc::new(Mutex::new(Context {
            parent_ctx: None,
            values: HashMap::new(),
        }))
    }

    /// Returns a new context with None parent context.
    #[allow(dead_code)]
    pub fn background() -> AppContext {
        Arc::new(Mutex::new(Context {
            parent_ctx: None,
            values: HashMap::new(),
        }))
    }

    /// Returns the user id from the context.
    /// Returns None if user is not found.
    pub fn user_id_from_context(ctx: AppContext) -> i64 {
        match Context::user_from_context(ctx) {
            Some(user) => user.id,
            None => 0,
        }
    }

    pub fn user_from_context(app: AppContext) -> Option<User> {
        let ctx = app.lock().unwrap();
        let user = ctx.value(CONTEXT_KEY_USER.to_string());
        match user {
            Some(Value::User(user)) => Some(user),
            _ => None,
        }
    }

    /// Returns the value stored in the context, if not found, returns None.
    pub fn value(&self, key: String) -> Option<Value> {
        let ctx = Some(self);

        let ctx = ctx.unwrap();

        if let Some(value) = ctx.values.get(&key) {
            return Some(value.clone());
        }

        None
    }

    /// Create a new context with the given user as the value of the key "user".
    pub fn with_user(ctx: AppContext, user: User) -> AppContext {
        Context::with_value(ctx, CONTEXT_KEY_USER.to_string(), Value::User(user))
    }

    /// Create a new context with the given parent context and key-value pairs.
    pub fn with_value(ctx: AppContext, key: String, value: Value) -> AppContext {
        let mut ctx = ctx.lock().unwrap();
        ctx.values.insert(key, value);

        Arc::new(Mutex::new(Context {
            parent_ctx: ctx.parent_ctx.clone(),
            values: ctx.values.clone(),
        }))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use openmusicgang_entity::user::User;

    #[test]
    fn with_value() {
        let ctx = Context::with_value(Context::background(), "val".to_string(), Value::Integer(32));

        assert_eq!(
            ctx.lock().unwrap().value("val".to_string()),
            Some(Value::Integer(32))
        );
    }

    #[test]
    fn test_with_user() {
        let mut user = User::new();
        user.name = "Bob Smith".to_string();

        let ctx = Context::with_user(Context::background(), user.clone());

        assert_eq!(
            ctx.lock().unwrap().value(CONTEXT_KEY_USER.to_string()),
            Some(Value::User(user))
        );
    }

    #[test]
    fn with_value_nested_ctxs() {
        let ctx = Context::with_value(
            Context::with_value(
                Context::with_value(Context::background(), "val".to_string(), Value::Integer(30)),
                "val2".to_string(),
                Value::Integer(31),
            ),
            "val3".to_string(),
            Value::Integer(32),
        );

        assert_eq!(
            ctx.lock().unwrap().value("val".to_string()),
            Some(Value::Integer(30))
        );

        assert_eq!(
            ctx.lock().unwrap().value("val2".to_string()),
            Some(Value::Integer(31))
        );

        assert_eq!(
            ctx.lock().unwrap().value("val3".to_string()),
            Some(Value::Integer(32))
        );
    }
}
