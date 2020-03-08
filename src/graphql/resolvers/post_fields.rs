use crate::db::enums::PostVoteType;
use crate::db::models::{Post, PostVote, User};
use crate::graphql::{Context, PostFields, VoteType};
use crate::map_to_field_result;

use juniper::{Executor, FieldError, FieldResult, ID};
use juniper_from_schema::{QueryTrail, Walked};

impl PostFields for Post {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<ID> {
        Ok(ID::new(self.id.to_string()))
    }

    fn field_content(&self, _: &Executor<'_, Context>) -> FieldResult<Option<String>> {
        if !self.content.is_empty() {
            return Ok(Some(self.content.clone()));
        }

        Ok(None)
    }

    fn field_html_content(&self, _: &Executor<'_, Context>) -> FieldResult<Option<String>> {
        if !self.content.is_empty() {
            return Ok(Some(self.content.clone()));
        }

        Ok(None)
    }

    fn field_upvotes(&self, executor: &Executor<'_, Context>) -> FieldResult<i32> {
        map_to_field_result!(executor
            .context()
            .db()
            .post_votes_count(&self, PostVoteType::UpVote))
    }

    fn field_downvotes(&self, executor: &Executor<'_, Context>) -> FieldResult<i32> {
        map_to_field_result!(executor
            .context()
            .db()
            .post_votes_count(&self, PostVoteType::DownVote))
    }

    fn field_author(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
    ) -> FieldResult<User> {
        map_to_field_result!(executor.context().db().user_by_id(self.author_id))
    }

    fn field_created_at(&self, _: &Executor<'_, Context>) -> FieldResult<String> {
        Ok(self.created_at.to_rfc3339())
    }

    fn field_user_vote(
        &self,
        executor: &Executor<'_, Context>,
        user_id: ID,
    ) -> FieldResult<Option<VoteType>> {
        Ok(executor
            .context()
            .db()
            .post_vote_by_user_id(&self, user_id.parse()?)
            .map(map_to_graphql_vote_type)
            .ok())
    }
}

fn map_to_graphql_vote_type(post_vote: PostVote) -> VoteType {
    match post_vote.vote_type {
        PostVoteType::DownVote => VoteType::Downvote,
        PostVoteType::UpVote => VoteType::Upvote,
    }
}
