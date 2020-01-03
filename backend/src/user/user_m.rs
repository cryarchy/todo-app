use juniper::{FieldResult, IntoFieldError, ID};
use validator::Validate;

use crate::app_context::AppContext;
use crate::errors::ValidationErrorsWrapper;
use crate::shared::Credentials;

pub struct UserM;

#[juniper::object(
    Context = AppContext
)]
impl UserM {
    #[graphql(description = "Create a new user")]
    fn new(context: &AppContext, user: Credentials) -> FieldResult<bool> {
        user.validate()
            .map_err(|e| ValidationErrorsWrapper(e).into_field_error())?;
        Ok(true)
    }

    #[graphql(description = "Remove an existing user")]
    fn remove(id: ID) -> FieldResult<bool> {
        Ok(true)
    }
}
