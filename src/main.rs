#[macro_use]
extern crate diesel;

mod db;
mod schema;
mod tasks;

fn main() {
    let mut conn = db::establish_connection();

    let command = tasks::features::create_task::command::Command {
        description: "Teste".to_string(),
        title: "Teste".to_string(),
    };

    match tasks::features::create_task::command::create_task_command(&mut conn, command) {
				Ok(task) => {
					println!("Task criada: {:?}", task)
				},
				Err(_) => {
					println!("Erro ao criar task")
				},
		};
}
