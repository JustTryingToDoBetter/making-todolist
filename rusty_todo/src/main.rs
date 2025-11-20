// src/main.rs
mod db;
mod model;

use clap::{Parser, Subcommand};
use comfy_table::{presets::UTF8_FULL, Cell, Color, ContentArrangement, Table};
use db::TodoManager;
use anyhow::Result;

// Define the CLI structure
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add { 
        /// The task description
        message: String 
    },
    /// List all tasks
    List {
        /// Show only completed tasks
        #[arg(short, long)]
        completed: bool,
    },
    /// Mark a task as done
    Done { 
        /// The ID of the task
        id: i32 
    },
    /// Delete a task
    Delete { 
        /// The ID of the task
        id: i32 
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    // Persist data to a file named 'todo.db' in the current directory
    let manager = TodoManager::new("todo.db")?;

    match &cli.command {
        Commands::Add { message } => {
            manager.add_todo(message)?;
            println!("✅ Todo added successfully!");
        }
        Commands::List { completed } => {
            let todos = manager.list_todos()?;
            if todos.is_empty() {
                println!("No tasks found. Go have a coffee! ☕");
                return Ok(());
            }

            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec!["ID", "Status", "Task", "Created At"]);

            for todo in todos {
                // Filter based on flag (optional logic)
                if *completed && !todo.completed { continue; }

                let status_cell = if todo.completed {
                    Cell::new("DONE").fg(Color::Green)
                } else {
                    Cell::new("PENDING").fg(Color::Yellow)
                };

                table.add_row(vec![
                    Cell::new(todo.id),
                    status_cell,
                    Cell::new(&todo.message),
                    Cell::new(todo.created_at.format("%Y-%m-%d %H:%M").to_string()),
                ]);
            }
            println!("{table}");
        }
        Commands::Done { id } => {
            if manager.complete_todo(*id)? {
                println!("🎉 Task {} marked as done!", id);
            } else {
                eprintln!("❌ Task {} not found.", id);
            }
        }
        Commands::Delete { id } => {
            if manager.delete_todo(*id)? {
                println!("🗑️ Task {} deleted.", id);
            } else {
                eprintln!("❌ Task {} not found.", id);
            }
        }
    }

    Ok(())
}