#[allow(dead_code)]
pub struct Migration<'a> {
    pub name: &'a str,
    pub query: &'a str,
}

#[allow(dead_code)]
pub fn get_migrations_list<'a>() -> Vec<Migration<'a>> {
    vec![Migration {
        name: "000-create_users_table",
        query: "CREATE TABLE users(
                    id BIGSERIAL PRIMARY KEY,
                    name VARCHAR(255) NOT NULL,
                    email VARCHAR(255) UNIQUE NULL,
                    password VARCHAR(255) NULL,
                    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
                );",
    }]
}
