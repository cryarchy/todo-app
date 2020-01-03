extern crate validator;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, Debug, juniper::GraphQLInputObject)]
pub struct Credentials {
    #[validate(length(min = 2, message = "Username must be longer than 1 character!"))]
    username: String,
    #[validate(length(min = 7, message = "Password must be longer than 7 characters!"))]
    password: String,
}