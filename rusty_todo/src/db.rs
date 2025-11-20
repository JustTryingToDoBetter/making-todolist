// src/db.rs
use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use crate::model::Todo;
use chrono::Local;

pub struct TodoManager {
    conn: Connection,
}

impl TodoManager {
    // Initialize DB and create table if it doesn't exist
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)
            .context("Failed to open database connection")?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY,
                message TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL
            )",
            [],
        ).context("Failed to create tables")?;

        Ok(Self { conn })
    }

    pub fn add_todo(&self, message: &str) -> Result<()> {
        let now = Local::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO todos (message, completed, created_at) VALUES (?1, ?2, ?3)",
            params![message, false, now],
        ).context("Failed to insert todo")?;
        Ok(())
    }

    pub fn list_todos(&self) -> Result<Vec<Todo>> {
        let mut stmt = self.conn.prepare("SELECT id, message, completed, created_at FROM todos")?;
        
        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?;

        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo?);
        }
        Ok(todos)
    }

    pub fn complete_todo(&self, id: i32) -> Result<bool> {
        let rows_affected = self.conn.execute(
            "UPDATE todos SET completed = 1 WHERE id = ?1",
            params![id],
        ).context("Failed to update todo")?;
        
        Ok(rows_affected > 0)
    }

    pub fn delete_todo(&self, id: i32) -> Result<bool> {
        let rows_affected = self.conn.execute(
            "DELETE FROM todos WHERE id = ?1",
            params![id],
        ).context("Failed to delete todo")?;

        Ok(rows_affected > 0)
    }
}