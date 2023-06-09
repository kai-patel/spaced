// @generated automatically by Diesel CLI.

diesel::table! {
    users (idx) {
        name -> Text,
        display_name -> Text,
        password_hash -> Nullable<Text>,
        email -> Nullable<Text>,
        federation_id -> Text,
        inbox -> Text,
        outbox -> Text,
        local -> Bool,
        public_key -> Text,
        private_key -> Nullable<Text>,
        last_refreshed_at -> Timestamptz,
        id -> Text,
        idx -> Int4,
    }
}
