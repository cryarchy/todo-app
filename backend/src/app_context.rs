use std::sync::{Arc, Mutex};

use juniper::Context;
use mongodb::Database;

use crate::{shared::Collections, user::User};

#[derive(Debug)]
pub struct AppContext {
    pub db: Database,
    pub session: Arc<Mutex<Option<User>>>,
    pub collections: Collections,
}

impl Context for AppContext {}
