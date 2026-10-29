// @generated automatically by Diesel CLI.

diesel::table! {
    addresses (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 100]
        receiver_name -> Varchar,
        #[max_length = 10]
        phone -> Varchar,
        #[max_length = 100]
        province -> Varchar,
        #[max_length = 100]
        district -> Nullable<Varchar>,
        #[max_length = 100]
        ward -> Varchar,
        #[max_length = 255]
        street -> Varchar,
        is_default -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

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
        #[max_length = 50]
        role -> Varchar,
    }
}

diesel::joinable!(addresses -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(addresses, users,);
