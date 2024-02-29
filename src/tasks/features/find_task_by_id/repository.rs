use crate::tasks::features::create_task::model::Task;

use diesel::prelude::*;
use diesel::result::Error;

pub fn find_task_by_id(conn: &mut PgConnection, task_id: i32) -> Result<Task, Error> {
    use crate::schema::tasks::dsl::*;

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
