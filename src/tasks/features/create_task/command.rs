use crate::tasks::domain::task;
use crate::tasks::features::create_task::repository;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub title: String,
    pub description: String,
}

pub fn create_task_command(
    conn: &mut PgConnection,
    command: Command,
) -> Result<task::Task, String> {
    println!("Creating task: {:?}", command);

    let mut task = task::Task::new(command.title, command.description);

    let created = repository::create(conn, &task);

    match created {
        Ok(created_task) => {
            task::Task::set_id(&mut task, created_task.id);
            Ok(task)
        }
        Err(e) => {
            println!("Error saving new task: {:?}", e);

            Err("Error saving new task".to_string())
        }
    }
}
