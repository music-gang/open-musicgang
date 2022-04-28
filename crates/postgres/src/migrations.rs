#[allow(dead_code)]
pub struct Migration<'a> {
    pub name: &'a str,
    pub query: &'a str,
}

#[allow(dead_code)]
pub fn migrations<'a>() -> Vec<Migration<'a>> {
    vec![
        Migration {
            name: "000-create_users_table",
            query: "CREATE TABLE users(
                    id BIGSERIAL PRIMARY KEY,
                    name VARCHAR(255) NOT NULL,
                    email VARCHAR(255) UNIQUE NULL,
                    password VARCHAR(255) NULL,
                    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
                );",
        },
        Migration {
            name: "001-create_auths_table",
            query: "CREATE TABLE auths(
                    id BIGSERIAL PRIMARY KEY,
                    user_id BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
                    source VARCHAR(255) NOT NULL,
                    source_id VARCHAR(255) NULL,
                    access_token VARCHAR(255) NULL,
                    refresh_token VARCHAR(255) NULL,
                    expiry TIMESTAMPTZ NULL,
                    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

                    UNIQUE(user_id, source),
                    -- one source per user

                    UNIQUE(source, source_id)
                    -- one auth per source user
                );",
        },
    ]
}
