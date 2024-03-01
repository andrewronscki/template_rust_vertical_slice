use waiter_di::*;

use crate::shared::di_container;
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

	pub fn query(&self, id: i32) -> Result<task::Task, String> {
		println!("Finding task: {:?}", id);

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
        Err(e) => {
            println!("Error find task: {:?}", e);

            Err("Error find task".to_string())
        }
    }
	
	}
}

