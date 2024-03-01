use diesel::prelude::*;
use diesel::result::Error;

use waiter_di::*;

use crate::shared::app_state::AppState;
use crate::tasks::domain::task;
use crate::tasks::features::create_task::model::{NewTask, Task};
use crate::shared::schema::tasks::dsl::*;

pub trait TCreateTaskRepository: Send {
    fn create(&self, task: &task::Task) -> Result<Task, Error>;
}

#[component]
pub struct CreateTaskRepository {}

#[provides]
impl TCreateTaskRepository for CreateTaskRepository {
    fn create(&self, task: &task::Task) -> Result<Task, Error> {
				let mut conn = AppState::get_instance().db_pool.get().expect("Failed to get db connection");

        let new_task = NewTask {
            title: &task.title,
            description: &task.description,
            status: &task.status.as_str(),
            removed: task.removed,
        };

        let created = diesel::insert_into(tasks)
            .values(&new_task)
            .get_result::<Task>(&mut conn);

        match created {
            Ok(created_task) => {
                println!("Created task: {:?}", created_task);
                Ok(created_task)
            }
            Err(e) => {
                println!("Error saving new task: {:?}", e);

                Err(e)
            }
        }
    }
}
