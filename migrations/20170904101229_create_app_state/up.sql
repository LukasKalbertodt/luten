create type app_state as enum ('preparation', 'running', 'frozen');

create table current_app_state (
    -- Artificial PK to ensure this table only stores one single row
    id bool
        primary key
        default TRUE,

    -- The state.
    state app_state
        not null,

    -- An optional reason why the app is in the current state. This is shown
    -- to the user for the 'frozen' state.
    reason text,

    -- An estimate on when the state will switch again. This is shown to the
    -- user for the 'frozen' state.
    next_state_switch timestamptz,


    -- Make sure the id is true (combined with the unique-contraints, this
    -- means that there is only one row).
    constraint current_app_state_one_row CHECK (id)
);

insert into current_app_state
    (state, reason)
    values ('frozen', 'Default state after database setup');
