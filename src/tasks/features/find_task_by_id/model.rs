use crate::schema::tasks;

#[derive(Debug, Queryable, AsChangeset)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: String,
    pub removed: bool,
}
