use crate::db::models::{Post, User};
use crate::graphql::{Context, UserFields};
use crate::map_to_field_result;

use juniper::{Executor, FieldError, FieldResult, ID};
use juniper_from_schema::{QueryTrail, Walked};

impl UserFields for User {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<ID> {
        Ok(ID::new(self.id.to_string()))
    }

    fn field_name(&self, _: &Executor<'_, Context>) -> FieldResult<String> {
        Ok(self.username.clone())
    }

    fn field_handle(&self, _: &Executor<'_, Context>) -> FieldResult<String> {
        Ok(self.handle.clone())
    }

    fn field_number_of_posts(&self, executor: &Executor<'_, Context>) -> FieldResult<i32> {
        map_to_field_result!(executor.context().db().get_user_posts_count(self))
    }

    fn field_profile_picture(&self, _: &Executor<'_, Context>) -> FieldResult<Option<String>> {
        Ok(self.profile_picture.clone())
    }

    fn field_posts(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Post, Walked>,
        first: i32,
        offset: i32,
    ) -> FieldResult<Option<Vec<Option<Post>>>> {
        map_to_field_result!(executor.context().db().user_posts(&self, first, offset))
    }

    fn field_post_count(&self, executor: &Executor<'_, Context>) -> FieldResult<i32> {
        self.field_number_of_posts(executor)
    }

    fn field_joined_at(&self, _: &Executor<'_, Context>) -> FieldResult<String> {
        Ok(self.created_at.to_string())
    }

    fn field_friends(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
        first: i32,
        offset: i32,
    ) -> FieldResult<Option<Vec<Option<User>>>> {
        map_to_field_result!(executor.context().db().user_friends(&self, first, offset))
    }

    fn field_friend_count(&self, executor: &Executor<'_, Context>) -> FieldResult<i32> {
        map_to_field_result!(executor.context().db().user_friends_count(&self))
    }

    fn field_points(&self, _: &Executor<'_, Context>) -> FieldResult<i32> {
        Ok(self.rankpoints)
    }

    // groups

    fn field_group_count(&self, executor: &Executor<'_, Context>) -> FieldResult<i32> {
        map_to_field_result!(executor.context().db().user_groups_count(&self))
    }

    fn field_level(&self, _: &Executor<'_, Context>) -> FieldResult<i32> {
        Ok((self.rankpoints as f32 / 100.0).ceil() as i32)
    }
}

//"the groups the user has joined"
//groups(first: Int=10, offset: Int=0): [Group]
//
//"The numbef of groups the user has joined"
//groupCount: Int!
