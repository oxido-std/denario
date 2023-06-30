-- Your SQL goes here
CREATE TABLE categories(
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_DATE,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_DATE,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
)

-- SELECT diesel_manage_updated_at('categories');