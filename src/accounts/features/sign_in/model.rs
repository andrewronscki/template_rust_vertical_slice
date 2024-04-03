use crate::shared::schema::accounts;

#[derive(Debug, Queryable, AsChangeset)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}
