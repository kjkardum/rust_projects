table! {
    urls (id) {
        id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        long_url -> Varchar,
        short_url -> Text,
        uses -> Int4,
    }
}

table! {
    users (id) {
        id -> Nullable<Int4>,
        username -> Varchar,
        password_hash -> Text,
        is_admin -> Bool,
    }
}

joinable!(urls -> users (user_id));

allow_tables_to_appear_in_same_query!(urls, users,);
