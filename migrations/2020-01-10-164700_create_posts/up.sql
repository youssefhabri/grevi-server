create table if not exists posts
(
    id         serial                   not null
        constraint posts_pkey
            primary key,
    content    varchar(2048)            not null,
    author_id  integer                  not null
        constraint posts_author_id_fkey
            references users
            on update cascade,
    created_at timestamp with time zone not null,
    updated_at timestamp with time zone not null
);