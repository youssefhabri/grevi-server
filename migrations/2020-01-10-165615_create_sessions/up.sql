create table if not exists "Sessions"
(
    sid         varchar(36)              not null
        constraint "Sessions_pkey"
            primary key,
    expires     timestamp with time zone,
    data        text,
    "createdAt" timestamp with time zone not null,
    "updatedAt" timestamp with time zone not null
);
