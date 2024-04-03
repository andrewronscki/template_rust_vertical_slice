use axum::http::StatusCode;
use utoipa::ToSchema;
use waiter_di::*;

use crate::{
    accounts::domain::account,
    shared::{di_container, exception_filter::CustomError},
};

use serde::{Deserialize, Serialize};

use super::repository::TSignInRepository;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub id: i32,
}

#[module]
pub struct QueryHandler {
    repo: Box<dyn TSignInRepository>,
}

impl QueryHandler {
    pub fn new() -> Self {
        let mut container = di_container::get::<profiles::Default>();
        Provider::<QueryHandler>::create(&mut container)
    }

    pub fn query(&self, query: Query) -> Result<account::Account, CustomError> {
        let account_found = self.repo.find_by_id(query.id);

        let account = match account_found {
            Ok(account) => account,

            Err(_) => {
                return Err(CustomError {
                    message: "User not found error".into(),
                    name: "UserNotFoundError".into(),
                    status: StatusCode::NOT_FOUND,
                })
            }
        };

        Ok(account)
    }
}
