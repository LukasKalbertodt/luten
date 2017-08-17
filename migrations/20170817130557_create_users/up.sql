create table users (
    id bigserial
        primary key,

    -- We don't limit the length of the username here, because we expect that
    -- users cannot choose their own username anyway. Thus, huge strings in the
    -- database are a bug and not an attack.
    username text
        not null,

    -- Not limited in length, see above.
    name text
);

create unique index users_unique_username_idx on users (username);
