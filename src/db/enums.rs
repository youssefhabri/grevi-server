//create type votetype as enum ('DOWNVOTE', 'UPVOTE');
//create type posttype as enum ('MISC', 'ACTION', 'IMAGE', 'TEXT');
//create type requesttype as enum ('FRIENDREQUEST');
//create type enum_post_votes_vote_type as enum ('UPVOTE', 'DOWNVOTE');
//create type enum_requests_request_type as enum ('FRIENDREQUEST', 'GROUPINVITE', 'EVENTINVITE');

#[derive(Copy, Clone, Debug, Eq, PartialEq, DbEnum)]
#[PgType = "enum_requests_request_type"]
#[DieselType = "EnumRequestType"]
pub enum RequestType {
    #[db_rename = "EVENTINVITE"]
    EventInvite,
    #[db_rename = "FRIENDREQUEST"]
    FriendRequest,
    #[db_rename = "GROUPINVITE"]
    GroupInvite,
}

#[derive(Copy, Clone, Debug, DbEnum, Eq, PartialEq)]
#[PgType = "enum_post_votes_vote_type"]
#[DieselType = "EnumPostVoteType"]
pub enum PostVoteType {
    #[db_rename = "DOWNVOTE"]
    DownVote,
    #[db_rename = "UPVOTE"]
    UpVote,
}
