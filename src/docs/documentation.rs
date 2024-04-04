use utoipa::OpenApi;

use crate::tasks::{
    domain::task::{Status, Task},
    features::create_task::command::CreateTaskCommand,
};

use crate::accounts::{
	domain::account::{Account, Tokens},
	features::{ sign_up::command::SignUpCommand, sign_in::command::SignInCommand, refresh_token::command::RefreshCommand },
};

#[derive(OpenApi)]
#[openapi(
	paths(
		crate::tasks::features::create_task::route::create_task, 
		crate::tasks::features::find_task_by_id::route::find_task_by_id,
		crate::accounts::features::sign_up::route::sign_up,
		crate::accounts::features::sign_in::route::sign_in,
		crate::accounts::features::refresh_token::route::refresh,
		crate::accounts::features::me::route::me
	),
	components(
		schemas(CreateTaskCommand, Task, Status, Account, Tokens, SignUpCommand, SignInCommand, RefreshCommand)
	),
	tags(
		(name = "tasks", description = "Funcionalidades de tarefas"),
		(name = "auth", description = "Funcionalidades de autenticação")
	)
)]
pub struct ApiDoc;
