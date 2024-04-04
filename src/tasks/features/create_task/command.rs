use axum::http::StatusCode;
use utoipa::ToSchema;
use waiter_di::*;

use crate::{
    shared::{di_container, exception_filter::CustomError},
    tasks::domain::task,
};

use serde::{Deserialize, Serialize};

use super::repository::TCreateTaskRepository;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateTaskCommand {
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

    pub fn command(&self, command: CreateTaskCommand) -> Result<task::Task, CustomError> {
        let mut task = task::Task::new(command.title, command.description);

        let created = self.repo.create(&task);

        match created {
            Ok(created_task) => {
                task::Task::set_id(&mut task, created_task.id);
                Ok(task)
            }
            Err(_) => Err(CustomError {
                message: "Create task error".into(),
                name: "CreateTaskError".into(),
                status: StatusCode::BAD_REQUEST,
            }),
        }
    }
}
