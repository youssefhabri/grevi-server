create table if not exists event_participants
(
    user_id    integer                  not null
        constraint event_participants_user_id_fkey
            references users
            on update cascade on delete cascade,
    event_id   integer                  not null
        constraint event_participants_event_id_fkey
            references events
            on update cascade on delete cascade,
    created_at timestamp with time zone not null,
    updated_at timestamp with time zone not null,
    constraint event_participants_pkey
        primary key (user_id, event_id)
);