// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        description -> Varchar,
        #[max_length = 255]
        status -> Varchar,
        removed -> Bool,
    }
}
