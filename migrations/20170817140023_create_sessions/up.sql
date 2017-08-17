create table sessions (
    -- The session ID.
    --
    -- It is exactly 16 bytes long, as also specified in `config.rs`.
    id bytea
        primary key
        check (octet_length(id) = 16),

    -- The owner of the session
    user_id bigint
        not null
        references users(id)
            on delete cascade
            on update cascade,

    -- When the session was created
    birth timestamptz
        not null
        default now()
);
