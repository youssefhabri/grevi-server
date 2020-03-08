create table if not exists post_votes
(
    vote_type  enum_post_votes_vote_type default 'UPVOTE'::enum_post_votes_vote_type not null,
    user_id    integer                                                               not null
        constraint post_votes_user_id_fkey
            references users
            on update cascade on delete cascade,
    post_id    integer                                                               not null
        constraint post_votes_post_id_fkey
            references posts
            on update cascade on delete cascade,
    created_at timestamp with time zone                                              not null,
    updated_at timestamp with time zone                                              not null,
    constraint post_votes_pkey
        primary key (user_id, post_id)
);