// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 256]
        first_name -> Varchar,
        #[max_length = 256]
        last_name -> Varchar,
        #[max_length = 256]
        email -> Varchar,
        #[max_length = 256]
        password_hash -> Varchar,
    }
}
