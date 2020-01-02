use juniper::{FieldResult, ID};

use crate::app::app_context::AppContext;
use crate::app::todo::Todo;

pub struct TodoM;

#[juniper::object(
    Context = AppContext,
)]
impl TodoM {
    #[graphql(description = "Add a new todo")]
    fn add(context: &AppContext, title: String) -> FieldResult<Todo> {
        Ok(Todo {
            id: ID::new("1"),
            title: "New todo".to_owned(),
            completed: false,
            owner: ID::new("owner"),
        })
    }

    #[graphql(description = "Remove an existing todo")]
    fn remove(context: &AppContext, id: ID) -> FieldResult<bool> {
        Ok(true)
    }

    #[graphql(description = "Update a todo's title")]
    fn update(context: &AppContext, id: ID, title: String) -> FieldResult<bool> {
        Ok(true)
    }

    #[graphql(description = "Mark a todo as complete")]
    fn mark_as_complete(context: &AppContext, id: ID) -> FieldResult<bool> {
        Ok(true)
    }

    #[graphql(description = "Mark all saved todos as complete")]
    fn mark_all_as_complete(context: &AppContext) -> FieldResult<bool> {
        Ok(true)
    }
}
