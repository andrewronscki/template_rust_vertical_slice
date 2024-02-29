use crate::tasks::domain::task;
use crate::tasks::features::find_task_by_id::repository;

use diesel::prelude::*;

pub fn find_task_query(conn: &mut PgConnection, id: i32) -> Result<task::Task, String> {
    println!("Finding task: {:?}", id);

    let found_task = repository::find_task_by_id(conn, id);

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
        Err(e) => {
            println!("Error find task: {:?}", e);

            Err("Error find task".to_string())
        }
    }
}
