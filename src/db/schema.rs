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
    prep_student_preferences (user_id) {
        user_id -> Int8,
        partner -> Nullable<Text>,
        prefers_english -> Bool,
    }
}
joinable!(prep_student_preferences -> users(user_id));

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
    use db::types::TimeslotRating;

    timeslot_ratings (user_id, timeslot_id) {
        user_id -> Int8,
        timeslot_id -> Int2,
        rating -> TimeslotRating,
    }
}
joinable!(timeslot_ratings -> users(user_id));
joinable!(timeslot_ratings -> timeslots(timeslot_id));

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
