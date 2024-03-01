use crate::shared::schema::tasks;

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub status: &'a str,
    pub removed: bool,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: String,
    pub removed: bool,
}
