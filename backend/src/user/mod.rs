use juniper::ID;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, juniper::GraphQLObject, Debug)]
#[graphql(description = "An application user")]
pub struct User {
    pub id: ID,
    pub username: String,
}

mod user_m;

pub use crate::user::user_m::UserM;
