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

    /// Returns the value stored in the context, if not found, returns None.
    pub fn value(&self, key: String) -> Option<Value> {
        let ctx = Some(self);

        while ctx.is_some() {
            let ctx = ctx.unwrap();

            if let Some(value) = ctx.values.get(&key) {
                return Some(value.clone());
            }

            ctx.parent_ctx.as_ref().map(|ctx| ctx.clone());
        }

        None
    }

    /// Create a new context with the given parent context and key-value pairs.
    pub fn with_value(ctx: AppContext, key: String, value: Value) -> AppContext {
        let mut values = ctx.lock().unwrap().values.clone();
        values.insert(key, value);

        Arc::new(Mutex::new(Context {
            parent_ctx: ctx.lock().unwrap().parent_ctx.clone(),
            values,
        }))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn with_value() {
        let ctx = Context::with_value(Context::background(), "val".to_string(), Value::Integer(32));

        assert_eq!(
            ctx.lock().unwrap().value("val".to_string()),
            Some(Value::Integer(32))
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
