use crate::tasks::features::create_task::model::Task;

use diesel::prelude::*;
use diesel::result::Error;

use waiter_di::*;

pub trait TFindTaskByIdRepository: Send {
    fn fin_by_id(&self, conn: &mut PgConnection, task_id: i32) -> Result<Task, Error>;
}

#[component]
pub struct FindTaskByIdRepository {}

#[provides]
impl TFindTaskByIdRepository for FindTaskByIdRepository {
    fn fin_by_id(&self, conn: &mut PgConnection, task_id: i32) -> Result<Task, Error> {
        use crate::shared::schema::tasks::dsl::*;

        let task_found = tasks.find(task_id).get_result::<Task>(conn);

        match task_found {
            Ok(task_found) => {
                println!("Task found: {:?}", task_found);
                Ok(task_found)
            }
            Err(e) => {
                println!("Error find task: {:?}", e);

                Err(e)
            }
        }
    }
}
