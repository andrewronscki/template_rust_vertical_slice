use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
