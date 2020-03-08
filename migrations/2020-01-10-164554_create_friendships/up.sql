create table if not exists friendships
(
    user_id    integer                  not null
        constraint friendships_user_id_fkey
            references users
            on update cascade on delete cascade,
    friend_id  integer                  not null
        constraint friendships_friend_id_fkey
            references users
            on update cascade on delete cascade,
    created_at timestamp with time zone not null,
    updated_at timestamp with time zone not null,
    constraint friendships_pkey
        primary key (user_id, friend_id)
);