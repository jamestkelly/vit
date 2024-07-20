// @generated automatically by Diesel CLI.

diesel::table! {
    branches (name) {
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 40]
        head_hash -> Nullable<Varchar>,
    }
}

diesel::table! {
    commit_parents (child_hash, parent_hash) {
        #[max_length = 40]
        child_hash -> Varchar,
        #[max_length = 40]
        parent_hash -> Varchar,
    }
}

diesel::table! {
    commits (hash) {
        #[max_length = 40]
        hash -> Varchar,
        message -> Nullable<Text>,
        #[max_length = 100]
        author -> Nullable<Varchar>,
        timestamp -> Nullable<Timestamp>,
        content -> Nullable<Jsonb>,
    }
}

diesel::table! {
    files (id) {
        id -> Uuid,
        url -> Text,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 40]
        commit_hash -> Nullable<Varchar>,
    }
}

diesel::table! {
    notes (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        description -> Varchar,
        #[max_length = 40]
        commit_hash -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id, username) {
        id -> Uuid,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        password_hash -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        avatar -> Nullable<Text>,
    }
}

diesel::joinable!(branches -> commits (head_hash));
diesel::joinable!(files -> commits (commit_hash));
diesel::joinable!(notes -> commits (commit_hash));

diesel::allow_tables_to_appear_in_same_query!(
    branches,
    commit_parents,
    commits,
    files,
    notes,
    users,
);
