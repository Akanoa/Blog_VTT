// @generated automatically by Diesel CLI.

diesel::table! {
    posts (uuid) {
        uuid -> Text,
        author_uuid -> Text,
        title -> Text,
        content -> Text,
    }
}

diesel::table! {
    users (uuid) {
        uuid -> Text,
        login -> Text,
        hash -> Text,
    }
}

diesel::joinable!(posts -> users (author_uuid));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
