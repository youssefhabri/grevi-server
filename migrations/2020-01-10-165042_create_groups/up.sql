create table if not exists groups
(
    id         serial                   not null
        constraint groups_pkey
            primary key,
    name       varchar(255)             not null,
    creator_id integer                  not null
        constraint groups_creator_id_fkey
            references users
            on update cascade on delete cascade,
    chat_id    integer                  not null
        constraint groups_chat_id_fkey
            references chat_rooms
            on update cascade,
    created_at timestamp with time zone not null,
    updated_at timestamp with time zone not null
);