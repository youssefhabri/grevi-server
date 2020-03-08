use crate::db::models::User;
use crate::graphql::{Context, QueryFields};

use juniper::{Executor, FieldError, FieldResult, ID};
use juniper_from_schema::{QueryTrail, Walked};

pub struct Query;

impl QueryFields for Query {
    fn field_get_user(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, User, Walked>,
        user_id: Option<ID>,
        handle: Option<String>,
    ) -> FieldResult<Option<User>> {
        if let Some(handle) = handle {
            return Ok(executor.context().db().user_by_handler(handle).ok());
        } else if let Some(id) = user_id {
            return match id.parse() {
                Ok(id) => Ok(executor.context().db().user_by_id(id).ok()),
                Err(_) => Err(FieldError::from("Invalid user_id.")),
            };
        }

        Err(FieldError::from("No userId or handle provided."))
    }
}
