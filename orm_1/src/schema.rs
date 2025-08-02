// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        title1 -> Text,
        body -> Text,
        published -> Bool,
    }
}
