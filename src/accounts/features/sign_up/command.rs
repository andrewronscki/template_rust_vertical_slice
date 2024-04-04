use axum::http::StatusCode;
use utoipa::ToSchema;
use waiter_di::*;

use crate::{
    accounts::domain::account,
    shared::{di_container, exception_filter::CustomError},
};

use serde::{Deserialize, Serialize};

use super::repository::TSignUpRepository;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SignUpCommand {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[module]
pub struct CommandHandler {
    repo: Box<dyn TSignUpRepository>,
}

impl CommandHandler {
    pub fn new() -> Self {
        let mut container = di_container::get::<profiles::Default>();
        Provider::<CommandHandler>::create(&mut container)
    }

    pub fn command(&self, command: SignUpCommand) -> Result<account::Tokens, CustomError> {
        let mut account = account::Account::new(command.name, command.email);

        account::Account::set_password(&mut account, command.password);

        let created = self.repo.create(&account);

        match created {
            Ok(created_account) => {
                account::Account::set_id(&mut account, created_account.id);
            }
            Err(_) => {
                return Err(CustomError {
                    message: "Create Account error".into(),
                    name: "CreateAccountError".into(),
                    status: StatusCode::BAD_REQUEST,
                })
            }
        };

        account::Account::set_tokens(&mut account);

        match self.repo.update_one_by_id(&account) {
            Ok(affected) => {
                if !affected {
                    return Err(CustomError {
                        message: "Create account tokens error".into(),
                        name: "CreateAccountTokensError".into(),
                        status: StatusCode::BAD_REQUEST,
                    });
                }
            }
            Err(_) => {
                return Err(CustomError {
                    message: "Create account tokens error".into(),
                    name: "CreateAccountTokensError".into(),
                    status: StatusCode::BAD_REQUEST,
                })
            }
        };

        Ok(account::Tokens {
            access_token: account.access_token,
            refresh_token: account.refresh_token,
        })
    }
}
