use axum::http::StatusCode;
use waiter_di::*;

use crate::shared::di_container;
use crate::shared::exception_filter::CustomError;
use crate::tasks::domain::task;

use super::repository::TFindTaskByIdRepository;

#[module]
pub struct QueryHandler {
    repo: Box<dyn TFindTaskByIdRepository>,
}

impl QueryHandler {
    pub fn new() -> Self {
        let mut container = di_container::get::<profiles::Default>();
        Provider::<QueryHandler>::create(&mut container)
    }

    pub fn query(&self, id: i32) -> Result<task::Task, CustomError> {
        let found_task = self.repo.fin_by_id(id);

        match found_task {
            Ok(found_task) => {
                let task = task::Task {
                    id: Some(found_task.id),
                    title: found_task.title,
                    description: found_task.description,
                    status: task::Status::from_str(&found_task.status).expect("Invalid status"),
                    removed: found_task.removed,
                };
                Ok(task)
            }
            Err(e) => match e {
                diesel::result::Error::NotFound => Err(CustomError {
                    name: "TaskNotFoundError".into(),
                    message: "Task not found error".into(),
                    status: StatusCode::NOT_FOUND,
                }),
                _ => Err(CustomError {
                    name: "FindTaskByIdError".into(),
                    message: "Find task by id error".into(),
                    status: StatusCode::BAD_REQUEST,
                }),
            },
        }
    }
}
