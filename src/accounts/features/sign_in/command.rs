use axum::http::StatusCode;
use utoipa::ToSchema;
use waiter_di::*;

use crate::{
    accounts::domain::account::{self, Tokens},
    shared::{di_container, exception_filter::CustomError},
};

use serde::{Deserialize, Serialize};

use super::repository::TSignInRepository;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Command {
    pub email: String,
    pub password: String,
}

#[module]
pub struct CommandHandler {
    repo: Box<dyn TSignInRepository>,
}

impl CommandHandler {
    pub fn new() -> Self {
        let mut container = di_container::get::<profiles::Default>();
        Provider::<CommandHandler>::create(&mut container)
    }

    pub fn command(&self, command: Command) -> Result<account::Tokens, CustomError> {
        let account_found = self.repo.find_by_email(&command.email);

        let mut account = match account_found {
            Ok(account) => account,

            Err(_) => {
                return Err(CustomError {
                    message: "User not found error".into(),
                    name: "UserNotFoundError".into(),
                    status: StatusCode::NOT_FOUND,
                })
            }
        };

        let valid = account::Account::verify_password(&mut account, &command.password);

        if !valid {
            return Err(CustomError {
                message: "Sign in error".into(),
                name: "SignInError".into(),
                status: StatusCode::CONFLICT,
            });
        }

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

        Ok(Tokens {
            access_token: account.access_token,
            refresh_token: account.refresh_token,
        })
    }
}
