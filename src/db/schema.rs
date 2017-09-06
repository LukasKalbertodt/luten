table! {
    use diesel::types::*;
    use db::types::AppState;

    current_app_state (id) {
        id -> Bool,
        state -> AppState,
        reason -> Nullable<Text>,
        next_state_switch -> Nullable<Timestamptz>,
    }
}

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
    use diesel::types::*;
    use db::types::DayOfWeek;

    timeslots (id) {
        id -> Int2,
        day -> DayOfWeek,
        time -> Time,
    }
}

table! {
    use diesel::types::*;
    use db::types::UserRole;

    users (id) {
        id -> Int8,
        username -> Text,
        name -> Nullable<Text>,
        role -> UserRole,
    }
}
