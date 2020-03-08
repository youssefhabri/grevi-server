create table if not exists events
(
    id         serial                   not null
        constraint events_pkey
            primary key,
    name       varchar(255)             not null,
    due_date   timestamp with time zone not null,
    group_id   integer                  not null
        constraint events_group_id_fkey
            references groups
            on update cascade on delete cascade,
    created_at timestamp with time zone not null,
    updated_at timestamp with time zone not null
);