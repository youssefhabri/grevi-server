#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

mod db;
mod graphql;
mod macros;

use rocket::{response::content, State};

use crate::db::DbConn;
use crate::graphql::*;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: Context,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: Context,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .manage(Schema::new(Query, Mutation))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
