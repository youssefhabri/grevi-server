create table if not exists users
(
    id                serial                     not null
        constraint users_pkey
            primary key,
    username          varchar(128)               not null,
    handle            varchar(128)               not null
        constraint users_handle_key
            unique,
    email             varchar(128)               not null
        constraint users_email_key
            unique,
    password          varchar(128)               not null,
    rankpoints        integer default 0          not null,
    created_at        timestamp with time zone   not null,
    updated_at        timestamp with time zone   not null,
    frontend_settings json    default '{}'::json not null,
    auth_token        varchar(255),
    auth_expire       timestamp with time zone,
    is_admin          boolean not null default false,
    profile_picture   varchar(500)
);

create unique index if not exists users_auth_token_uindex
    on users (auth_token);