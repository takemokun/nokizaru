// @generated automatically by Diesel CLI.

diesel::table! {
    spaces (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
