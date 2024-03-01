use waiter_di::*;

use crate::{shared::di_container, tasks::domain::task};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::repository::TCreateTaskRepository;

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub title: String,
    pub description: String,
}

#[module]
pub struct CommandHandler {
    repo: Box<dyn TCreateTaskRepository>,
}

impl CommandHandler {
    pub fn new() -> Self {
        let mut container = di_container::get::<profiles::Default>();
        Provider::<CommandHandler>::create(&mut container)
    }

    pub fn command(&self, conn: &mut PgConnection, command: Command) -> Result<task::Task, String> {
        println!("Creating task: {:?}", command);

        let mut task = task::Task::new(command.title, command.description);

        let created = self.repo.create(conn, &task);

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
}
