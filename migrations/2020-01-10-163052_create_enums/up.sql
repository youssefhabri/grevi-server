create type votetype as enum ('DOWNVOTE', 'UPVOTE');

create type posttype as enum ('MISC', 'ACTION', 'IMAGE', 'TEXT');

create type requesttype as enum ('FRIENDREQUEST');

create type enum_post_votes_vote_type as enum ('UPVOTE', 'DOWNVOTE');

create type enum_requests_request_type as enum ('FRIENDREQUEST', 'GROUPINVITE', 'EVENTINVITE');