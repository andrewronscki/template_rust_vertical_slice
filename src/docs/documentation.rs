use utoipa::OpenApi;

use crate::tasks::{
    domain::task::{Status, Task},
    features::create_task::command::Command as CreateTaskCommand,
};

use crate::accounts::{
	domain::account::{Account, Tokens},
	features::{sign_up::command::Command as SignUpCommand, sign_in::command::Command as SignInCommand},
};

#[derive(OpenApi)]
#[openapi(
	paths(
		crate::tasks::features::create_task::route::create_task, 
		crate::tasks::features::find_task_by_id::route::find_task_by_id,
		crate::accounts::features::sign_up::route::sign_up,
		crate::accounts::features::sign_in::route::sign_in,
		crate::accounts::features::me::route::me
	),
	components(
		schemas(CreateTaskCommand, Task, Status, Account, Tokens, SignUpCommand, SignInCommand)
	),
	tags(
		(name = "tasks", description = "Funcionalidades de tarefas"),
		(name = "auth", description = "Funcionalidades de autenticação")
	)
)]
pub struct ApiDoc;
