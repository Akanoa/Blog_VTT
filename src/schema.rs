// @generated automatically by Diesel CLI.

diesel::table! {
    users (uuid) {
        uuid -> Text,
        login -> Text,
        hash -> Text,
    }
}
