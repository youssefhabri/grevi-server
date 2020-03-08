create table if not exists requests
(
    id           serial                                                                         not null
        constraint requests_pkey
            primary key,
    request_type enum_requests_request_type default 'FRIENDREQUEST'::enum_requests_request_type not null,
    sender_id    integer                                                                        not null
        constraint requests_sender_id_fkey
            references users
            on update cascade,
    receiver_id  integer                                                                        not null
        constraint requests_receiver_id_fkey
            references users
            on update cascade,
    created_at   timestamp with time zone                                                       not null,
    updated_at   timestamp with time zone                                                       not null
);