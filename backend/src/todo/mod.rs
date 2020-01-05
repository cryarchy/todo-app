use juniper::ID;
use serde::{Deserialize, Serialize};

#[derive(juniper::GraphQLObject, Deserialize, Serialize)]
pub struct Todo {
    #[serde(rename = "_id")]
    #[graphql(description = "The todo's database ID")]
    id: ID,
    #[graphql(description = "The todo's title")]
    title: String,
    #[graphql(description = "The todo's completion status")]
    completed: bool,
    #[graphql(description = "The creator of the todo")]
    owner: ID,
}

pub mod todo_m;
pub use todo_m::TodoM;

pub mod todo_q;
pub use todo_q::TodoQ;
