// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        access_token -> Nullable<Varchar>,
        #[max_length = 255]
        refresh_token -> Nullable<Varchar>,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    tasks,
);
