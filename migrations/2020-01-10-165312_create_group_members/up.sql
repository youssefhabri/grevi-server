create table if not exists group_members
(
    user_id    integer                  not null
        constraint group_members_user_id_fkey
            references users
            on update cascade on delete cascade,
    group_id   integer                  not null
        constraint group_members_group_id_fkey
            references groups
            on update cascade on delete cascade,
    created_at timestamp with time zone not null,
    updated_at timestamp with time zone not null,
    constraint group_members_pkey
        primary key (user_id, group_id)
);