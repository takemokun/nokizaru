// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        slack_user_id -> Text,
        slack_team_id -> Text,
        display_name -> Nullable<Text>,
        email -> Nullable<Text>,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
