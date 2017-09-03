create type user_role as enum ('admin', 'tutor', 'student');

alter table users
    add column role user_role
        not null
        default 'student';
