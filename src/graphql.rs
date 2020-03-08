mod resolvers;

use juniper_from_schema::graphql_schema_from_file;
use rocket::request::{FromRequest, Outcome, Request};

use crate::db::{
    models::{Post, User},
    Database, DbConn,
};

pub use resolvers::*;

graphql_schema_from_file!("assets/schema.graphql");

pub struct Context {
    db: Database,
}

impl juniper::Context for Context {}

impl<'a, 'r> FromRequest<'a, 'r> for Context {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Context, ()> {
        let db_conn = request.guard::<DbConn>()?;
        let db = Database::new(db_conn);
        Outcome::Success(Context { db })
    }
}

impl Context {
    pub fn db(&self) -> &Database {
        &self.db
    }
}
