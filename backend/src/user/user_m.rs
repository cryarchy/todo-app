use juniper::{FieldError, FieldResult, ID};
use validator::Validate;

use crate::app_context::AppContext;
use crate::errors::FromValidationErrors;
use crate::shared::{Credentials, SubTest, Test};

pub struct UserM;

#[juniper::object(
    Context = AppContext
)]
impl UserM {
    #[graphql(description = "Create a new user")]
    fn new(context: &AppContext, user: Credentials) -> FieldResult<Option<bool>> {
        user.validate()
            .map_err(|e| FieldError::from_validation_errors(e))?;
        Ok(Some(true))
    }

    #[graphql(description = "Remove an existing user")]
    fn remove(id: ID) -> FieldResult<bool> {
        let test = Test {
            sub_test: SubTest {
                username: "u".to_owned(),
                password: "pass".to_owned(),
            },
            sub_tests: vec![
                SubTest {
                    username: "username".to_owned(),
                    password: "password".to_owned(),
                },
                SubTest {
                    username: "u".to_owned(),
                    password: "pass".to_owned(),
                },
            ],
            password: "pass".to_owned(),
        };
        test.validate()
            .map_err(|e| FieldError::from_validation_errors(e))?;
        Ok(true)
    }
}
