use std::sync::{Arc, Mutex};
use crate::models::user::User;

#[derive(Clone, Default)]
pub struct AppState{
    pub users: Arc<Mutex<Vec<User>>>,
}

impl AppState{
    pub fn new() -> Self{
        Self{
            users:Arc::new(Mutex::new(Vec::new())),
        }
    }
}