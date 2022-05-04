/// delete_user_sql is a macro that generates a SQL query to delete a user.
#[macro_export]
macro_rules! delete_user_sql {
    () => {
        "DELETE FROM users WHERE id = $1"
    };
}

/// delete_user_params is a macro that returns a tuple of the parameters to be used in the delete_user_sql macro.
#[macro_export]
macro_rules! delete_user_params {
    ($id:expr) => {
        &[&$id]
    };
}

/// insert_user_sql is a macro that generates the SQL to insert a user into the database.
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
            .to_string()
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

/// select_users_sql is a macro that generates the SQL to select users from the database.
#[macro_export]
macro_rules! select_users_sql {
    ($whereConditions:expr,$limitOffsetConditions:expr) => {
        format!("
        SELECT 
            id,
            name,
            email,
            password,
            created_at,
            updated_at,
            COUNT(*) OVER() as count
        FROM users
        WHERE
        {}
        ORDER BY id ASC
        {}
        ", $whereConditions.join("\nAND "), $limitOffsetConditions)
    }
}
