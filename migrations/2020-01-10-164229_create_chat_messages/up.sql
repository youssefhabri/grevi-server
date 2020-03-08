create table if not exists chat_messages
(
    id         serial                   not null
        constraint chat_messages_pkey
            primary key,
    content    varchar(512)             not null,
    chat_id    integer                  not null
        constraint chat_messages_chat_id_fkey
            references chat_rooms
            on update cascade,
    author_id  integer                  not null
        constraint chat_messages_author_id_fkey
            references users
            on update cascade,
    created_at timestamp with time zone not null,
    updated_at timestamp with time zone not null
);