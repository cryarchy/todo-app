extern crate validator;

use bson;
use mongodb::{options::IndexModel, Database};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, Debug, juniper::GraphQLInputObject)]
pub struct Credentials {
    #[validate(length(min = 2, message = "Username must be longer than 1 character!"))]
    pub username: String,
    #[validate(length(min = 7, message = "Password must be longer than 7 characters!"))]
    pub password: String,
}

#[derive(Debug)]
pub struct Collections {
    pub user: Collection,
    pub todo: Collection,
}

#[derive(Debug)]
pub struct Collection {
    pub name: &'static str,
    pub indexes: Option<Vec<IndexModel>>,
}

impl Collections {
    pub fn new() -> Collections {
        Collections {
            user: Collection {
                name: "todo_users",
                indexes: Some(vec![IndexModel {
                    keys: doc! { "username": 1 },
                    options: Some(doc! { "unique": true }),
                }]),
            },
            todo: Collection {
                name: "todos",
                indexes: Some(vec![IndexModel {
                    keys: doc! { "title": 1 },
                    options: None,
                }]),
            },
        }
    }

    pub fn create_indexes(&self, _db: Database) {
        unimplemented!()
    }
}
