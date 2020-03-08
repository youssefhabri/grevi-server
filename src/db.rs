pub mod enums;
pub mod models;
pub mod schema;

use enums::PostVoteType;
use models::{Post, PostVote, User};

use diesel::dsl::array;
use diesel::{
    BelongingToDsl, ExpressionMethods, PgArrayExpressionMethods, PgConnection, QueryDsl,
    QueryResult, RunQueryDsl,
};
use rocket_contrib::database;

#[database("grevi_db")]
pub struct DbConn(PgConnection);

pub struct Database {
    db_conn: DbConn,
}

impl Database {
    pub fn new(db_conn: DbConn) -> Self {
        Database { db_conn }
    }

    pub fn conn(&self) -> &PgConnection {
        &self.db_conn.0
    }

    pub fn user_by_id(&self, user_id: i32) -> QueryResult<User> {
        use schema::users::dsl::*;
        users.find(user_id).first(self.conn())
    }

    pub fn user_by_handler(&self, user_handle: String) -> QueryResult<User> {
        use schema::users::dsl::*;
        users
            .filter(handle.eq(user_handle))
            .first::<User>(self.conn())
    }

    pub fn get_user_posts_count(&self, user: &User) -> QueryResult<i32> {
        Post::belonging_to(user)
            .count()
            .get_result(self.conn())
            .map(|count: i64| count as i32)
    }

    pub fn user_posts(
        &self,
        user: &User,
        first: i32,
        offset: i32,
    ) -> QueryResult<Option<Vec<Option<Post>>>> {
        let posts: Vec<Option<Post>> = Post::belonging_to(user)
            .limit(first as i64)
            .offset(offset as i64)
            .load::<Post>(self.conn())?
            .into_iter()
            .map(Some)
            .collect();

        Ok(if !posts.is_empty() { Some(posts) } else { None })
    }

    pub fn user_friends(
        &self,
        user: &User,
        first: i32,
        offset: i32,
    ) -> QueryResult<Option<Vec<Option<User>>>> {
        use schema::friendships::dsl::*;
        use schema::users::dsl::*;

        let friend_ids: Vec<i32> = friendships
            .select(friend_id)
            .filter(user_id.eq(user.id))
            .load::<i32>(self.conn())?;

        let friends: Vec<Option<User>> = users
            .filter(array((id,)).is_contained_by(friend_ids))
            .limit(first as i64)
            .offset(offset as i64)
            .load::<User>(self.conn())?
            .into_iter()
            .map(Some)
            .collect();

        Ok(if !friends.is_empty() {
            Some(friends)
        } else {
            None
        })
    }

    pub fn user_friends_count(&self, user: &User) -> QueryResult<i32> {
        use schema::friendships::dsl::*;
        use schema::users::dsl::*;

        let friend_ids: Vec<i32> = friendships
            .select(friend_id)
            .filter(user_id.eq(user.id))
            .load::<i32>(self.conn())?;

        users
            .filter(array((id,)).is_contained_by(friend_ids))
            .count()
            .get_result(self.conn())
            .map(|count: i64| count as i32)
    }

    pub fn user_groups_count(&self, user: &User) -> QueryResult<i32> {
        use schema::group_members::dsl::*;

        group_members
            .filter(user_id.eq(user.id))
            .count()
            .get_result(self.conn())
            .map(|count: i64| count as i32)
    }

    pub fn post_vote_by_user_id(&self, post: &Post, uid: i32) -> QueryResult<PostVote> {
        use schema::post_votes::dsl::*;

        post_votes
            .filter(post_id.eq(post.id))
            .filter(user_id.eq(uid))
            .first::<PostVote>(self.conn())
    }

    pub fn post_votes_count(&self, post: &Post, vtype: PostVoteType) -> QueryResult<i32> {
        use schema::post_votes::dsl::*;

        post_votes
            .filter(post_id.eq(post.id))
            .filter(vote_type.eq(vtype))
            .count()
            .get_result(self.conn())
            .map(|count: i64| count as i32)
    }
}
