use crate::tasks::domain::task;
use crate::tasks::features::create_task::model::{NewTask, Task};

use diesel::prelude::*;
use diesel::result::Error;

use waiter_di::*;

pub trait TCreateTaskRepository: Send {
    fn create(&self, conn: &mut PgConnection, task: &task::Task) -> Result<Task, Error>;
}

#[component]
pub struct CreateTaskRepository {}

#[provides]
impl TCreateTaskRepository for CreateTaskRepository {
    fn create(&self, conn: &mut PgConnection, task: &task::Task) -> Result<Task, Error> {
        use crate::shared::schema::tasks::dsl::*;

        let new_task = NewTask {
            title: &task.title,
            description: &task.description,
            status: &task.status.as_str(),
            removed: task.removed,
        };

        let created = diesel::insert_into(tasks)
            .values(&new_task)
            .get_result::<Task>(conn);

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
