// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Nullable<Varchar>,
        registered -> Timestamp,
        enc_pass -> Varchar,
        token -> Nullable<Varchar>,
    }
}
