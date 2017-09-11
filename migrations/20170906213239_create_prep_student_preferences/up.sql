create table prep_student_preferences (
    user_id bigint
        primary key
        references users(id)
            on delete cascade
            on update cascade,

    -- Students can optionally specify a partner
    partner text,

    -- Preferred language of the user
    prefers_english bool
        not null
);
