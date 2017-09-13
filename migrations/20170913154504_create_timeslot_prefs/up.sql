create type timeslot_rating as enum (
    'good',
    'tolerable',
    'bad'
);

create table timeslot_ratings (
    user_id bigint
        not null
        references users(id)
            on delete cascade
            on update cascade,

    -- Students can optionally specify a partner
    timeslot_id smallint
        not null
        references timeslots(id)
            on delete restrict
            on update cascade,

    -- Preferred language of the user
    rating timeslot_rating
        not null,

    primary key (user_id, timeslot_id)
);


