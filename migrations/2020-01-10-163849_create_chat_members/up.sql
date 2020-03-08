create table if not exists chat_members
(
    user_id    integer                  not null
        constraint chat_members_user_id_fkey
            references users
            on update cascade on delete cascade,
    chat_id    integer                  not null
        constraint chat_members_chat_id_fkey
            references chat_rooms
            on update cascade on delete cascade,
    created_at timestamp with time zone not null,
    updated_at timestamp with time zone not null,
    constraint chat_members_pkey
        primary key (user_id, chat_id)
);