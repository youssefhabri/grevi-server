create table if not exists chat_rooms
(
    id         serial                   not null
        constraint chat_rooms_pkey
            primary key,
    created_at timestamp with time zone not null,
    updated_at timestamp with time zone not null
);