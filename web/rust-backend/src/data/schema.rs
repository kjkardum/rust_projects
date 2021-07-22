table! {
    users (id) {
        id -> Nullable<Int4>,
        username -> Varchar,
        password_hash -> Text,
        is_admin -> Bool,
    }
}
