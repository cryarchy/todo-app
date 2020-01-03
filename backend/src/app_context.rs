use std::sync::{Arc, Mutex};

use juniper::Context;
use mongodb::Database;

use crate::user::User;

#[derive(Debug)]
pub struct AppContext {
    pub db: Database,
    pub session: Arc<Mutex<Option<User>>>,
}

impl Context for AppContext {}
