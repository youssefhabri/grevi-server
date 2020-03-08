use crate::graphql::{Context, MutationFields};
use juniper::{Executor, FieldResult};

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_noop(&self, _executor: &Executor<'_, Context>) -> FieldResult<&bool> {
        Ok(&true)
    }
}
