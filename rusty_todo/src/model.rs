// src/models.rs
use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct Todo {
    pub id: i32,
    pub message: String,
    pub completed: bool,
    pub created_at: DateTime<Local>,
}

impl Todo {
    pub fn new(id: i32, message: String, completed: bool, created_at_str: String) -> Self {
        // In a real app, you might handle parsing errors more gracefully
        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .unwrap_or_default()
            .with_timezone(&Local);

        Self {
            id,
            message,
            completed,
            created_at,
        }
    }
}