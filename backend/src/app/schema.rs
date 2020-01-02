use juniper::{FieldResult, RootNode, ID};

use crate::app::shared::Credentials;

use crate::app::todo::{TodoM, TodoQ};
use crate::app::user::{User, UserM};

use crate::app::app_context::AppContext;

#[derive(Debug)]
pub struct Queries;

#[juniper::object(
    Context = AppContext,
)]
impl Queries {
    fn todo() -> TodoQ {
        TodoQ {}
    }
}

#[derive(Debug)]
pub struct Mutations;

#[juniper::object(
    Context = AppContext,
)]
impl Mutations {
    fn user() -> UserM {
        UserM {}
    }

    fn todo() -> TodoM {
        TodoM {}
    }

    fn login(context: &AppContext, credentials: Credentials) -> FieldResult<User> {
        Ok(User {
            id: ID::new("id"),
            username: "Example User".to_owned(),
        })
    }

    fn logout() -> FieldResult<bool> {
        Ok(true)
    }
}

pub type Schema = RootNode<'static, Queries, Mutations>;

pub fn create_schema() -> Schema {
    Schema::new(Queries {}, Mutations {})
}
