create type day_of_week as enum (
    'monday',
    'tuesday',
    'wednesday',
    'thursday',
    'friday',
    'saturday',
    'sunday'
);

create table timeslots (
    id smallserial
        primary key,

    day day_of_week
        not null,

    time time
        not null
);

create unique index timeslots_unique_daytime_idx on timeslots (day, time);
