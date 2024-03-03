use diesel::prelude::*;
use diesel::result::Error;

use waiter_di::*;

use crate::shared::app_state::AppState;
use crate::shared::schema::tasks::dsl::*;
use crate::tasks::features::create_task::model::Task;

pub trait TFindTaskByIdRepository: Send {
    fn fin_by_id(&self, task_id: i32) -> Result<Task, Error>;
}

#[component]
pub struct FindTaskByIdRepository {}

#[provides]
impl TFindTaskByIdRepository for FindTaskByIdRepository {
    fn fin_by_id(&self, task_id: i32) -> Result<Task, Error> {
        let mut conn = AppState::get_instance()
            .db_pool
            .get()
            .expect("Failed to get db connection");

        let task_found = tasks.find(task_id).get_result::<Task>(&mut conn);

        match task_found {
            Ok(task_found) => {
                Ok(task_found)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}
