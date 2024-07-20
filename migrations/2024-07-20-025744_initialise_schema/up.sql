-- up.sql
-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE commits (
    hash VARCHAR(40) PRIMARY KEY,
    message TEXT,
    author VARCHAR(100),
    timestamp TIMESTAMP,
    content JSONB
);

CREATE TABLE commit_parents (
    child_hash VARCHAR(40),
    parent_hash VARCHAR(40),
    PRIMARY KEY (child_hash, parent_hash),
    FOREIGN KEY (child_hash) REFERENCES commits (hash) ON DELETE CASCADE,
    FOREIGN KEY (parent_hash) REFERENCES commits (hash) ON DELETE CASCADE
);

CREATE TABLE branches (
    name VARCHAR(100) PRIMARY KEY,
    head_hash VARCHAR(40),
    FOREIGN KEY (head_hash) REFERENCES commits (hash)
);

CREATE TABLE users (
    id UUID DEFAULT uuid_generate_v4 (),
    username VARCHAR(100) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    avatar TEXT,
    PRIMARY KEY (id, username)
);

CREATE TABLE files (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    url TEXT NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    commit_hash VARCHAR(40),
    FOREIGN KEY (commit_hash) REFERENCES commits (hash) ON DELETE CASCADE
);

CREATE TABLE notes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    name VARCHAR(100) NOT NULL,
    description VARCHAR(255) NOT NULL,
    commit_hash VARCHAR(40),
    FOREIGN KEY (commit_hash) REFERENCES commits (hash) ON DELETE CASCADE
);
