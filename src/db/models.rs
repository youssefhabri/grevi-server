use chrono::{DateTime, Utc};
use diesel::{Associations, Queryable};

use crate::db::enums::PostVoteType;
use crate::db::schema::*;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub handle: String,
    pub email: String,
    pub password: String,
    pub rankpoints: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub frontend_settings: serde_json::Value,
    pub auth_token: Option<String>,
    pub auth_expire: Option<DateTime<Utc>>,
    pub is_admin: bool,
    pub profile_picture: Option<String>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "author_id")]
pub struct Post {
    pub id: i32,
    pub content: String,
    pub author_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Post, foreign_key = "post_id")]
pub struct PostVote {
    pub vote_type: PostVoteType,
    pub user_id: i32,
    pub post_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "user_id")]
pub struct Friendship {
    pub user_id: i32,
    pub friend_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "creator_id")]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub creator_id: i32,
    pub chat_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
