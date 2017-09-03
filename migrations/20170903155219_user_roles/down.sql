alter table if exists users
    drop column if exists role;

drop type if exists user_role;
