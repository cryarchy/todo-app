use juniper::{FieldResult, ID};

use crate::app::app_context::AppContext;
use crate::app::shared::Credentials;

pub struct UserM;

#[juniper::object(
    Context = AppContext
)]
impl UserM {
    #[graphql(description = "Create a new user")]
    fn new(context: &AppContext, user: Credentials) -> FieldResult<bool> {
        Ok(true)
    }

    #[graphql(description = "Remove an existing user")]
    fn remove(id: ID) -> FieldResult<bool> {
        Ok(true)
    }
}
