use crate::tasks::domain::task;
use crate::tasks::features::create_task::model::{NewTask, Task};

use diesel::prelude::*;

#[derive(Debug)]
pub struct Command {
    pub title: String,
    pub description: String,
}

pub fn create_task_command(conn: &mut PgConnection, command: Command) -> Result<task::Task, ()> {
    println!("Creating task: {:?}", command);
    use crate::schema::tasks::dsl::*;

    let mut task = task::Task::new(command.title, command.description);

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
            task::Task::set_id(&mut task, created_task.id);
            Ok(task)
        }
        Err(e) => {
            println!("Error saving new task: {:?}", e);

            Err(())
        }
    }
}
