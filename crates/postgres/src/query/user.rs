#[macro_export]
macro_rules! insert_user_sql {
    () => {
        "INSERT INTO users (
            name,
            email,
            password,
            created_at,
            updated_at
        ) VALUES ( $1, $2, $3, $4, $5 ) RETURNING id"
    };
}

/// insert_user_params returns the parameters for an INSERT statement in users table.
#[macro_export]
macro_rules! insert_user_params {
    ($user:expr) => {
        &[
            &$user.name,
            &$user.email,
            &$user.password,
            &$user.created_at,
            &$user.updated_at,
        ]
    };
}
