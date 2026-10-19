// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 100]
        password_hash -> Varchar,
        #[max_length = 100]
        full_name -> Varchar,
        #[max_length = 50]
        user_name -> Varchar,
        #[max_length = 10]
        phone -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
