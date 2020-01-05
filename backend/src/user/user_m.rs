use bson;
use juniper::{self, FieldError, FieldResult, ID};
use validator::Validate;

use crate::app_context::AppContext;
use crate::errors::FromValidationErrors;
use crate::shared::Credentials;

pub struct UserM;

#[juniper::object(
    Context = AppContext
)]
impl UserM {
    #[graphql(description = "Create a new user")]
    fn new(context: &AppContext, user: Credentials) -> FieldResult<ID> {
        user.validate()
            .map_err(|e| FieldError::from_validation_errors(e))?;
        let doc: bson::ordered::OrderedDocument = bson::from_bson(bson::to_bson(&user)?)?;
        let collection = context.db.collection(context.collections.user.name);
        let saved_user = collection.find_one(Some(doc! {"username": user.username}), None)?;
        if saved_user.is_none() == true {
            collection
                .insert_one(doc.clone(), None)
                .map(|result| Ok(ID::new(result.inserted_id.to_string())))?
        } else {
            Err(FieldError::new(
                "A user with the provided username already exists!",
                juniper::Value::Null,
            ))
        }
    }

    #[graphql(description = "Remove an existing user")]
    fn remove(id: ID) -> FieldResult<bool> {
        Ok(true)
    }
}
