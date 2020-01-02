use juniper::{FieldResult, ID};

use crate::app::app_context::AppContext;
use crate::app::todo::Todo;

pub struct TodoQ;

#[juniper::object(
    Context = AppContext,
)]
impl TodoQ {
    #[graphql(description = "Get the list of saved todos")]
    #[graphql(arguments(category(
        description = "Filter the todos by category. Acceptable values: 'all|active|completed'"
    )))]
    fn get(context: &AppContext, category: String) -> FieldResult<Vec<Todo>> {
        Ok(vec![
            Todo {
                id: ID::new("1"),
                title: "Todo 1".to_owned(),
                completed: false,
                owner: ID::new("owner"),
            },
            Todo {
                id: ID::new("2"),
                title: "Todo 2".to_owned(),
                completed: false,
                owner: ID::new("owner"),
            },
        ])
    }

    fn remaining(context: &AppContext) -> FieldResult<i32> {
        Ok(3)
    }
}
