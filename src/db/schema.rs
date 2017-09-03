table! {
    passwords (user_id) {
        user_id -> Int8,
        hash -> Bpchar,
    }
}
joinable!(passwords -> users(user_id));

table! {
    sessions (id) {
        id -> Bytea,
        user_id -> Int8,
        birth -> Timestamptz,
    }
}
joinable!(sessions -> users(user_id));

table! {
    users (id) {
        id -> Int8,
        username -> Text,
        name -> Nullable<Text>,
    }
}
