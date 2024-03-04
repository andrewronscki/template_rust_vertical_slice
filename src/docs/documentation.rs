use utoipa::OpenApi;

use crate::tasks::{
    domain::task::{Status, Task},
    features::create_task::command::Command,
};

#[derive(OpenApi)]
#[openapi(
	paths(
		crate::tasks::features::create_task::route::create_task, 
		crate::tasks::features::find_task_by_id::route::find_task_by_id
	),
	components(
		schemas(Command, Task, Status)
	),
	tags(
		(name = "tasks", description = "Funcionalidades de tarefas")
	)
)]
pub struct ApiDoc;
