create table passwords (
    user_id bigint
        primary key
        references users(id)
            on delete cascade
            on update cascade,

    -- bcrypt hash string
    hash char(60)
        not null
);
