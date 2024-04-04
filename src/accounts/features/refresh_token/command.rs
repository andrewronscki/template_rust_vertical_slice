use axum::http::StatusCode;
use utoipa::ToSchema;
use waiter_di::*;

use crate::{
    accounts::domain::account::{self, Tokens},
    shared::{di_container, exception_filter::CustomError},
};

use serde::{Deserialize, Serialize};

use super::repository::TRefreshTokenRepository;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RefreshCommand {
    pub access_token: String,
    pub refresh_token: String,
}

#[module]
pub struct CommandHandler {
    repo: Box<dyn TRefreshTokenRepository>,
}

impl CommandHandler {
    pub fn new() -> Self {
        let mut container = di_container::get::<profiles::Default>();
        Provider::<CommandHandler>::create(&mut container)
    }

    pub fn command(&self, command: RefreshCommand) -> Result<account::Tokens, CustomError> {
        let account_found = self
            .repo
            .find_by_tokens(&command.access_token, &command.refresh_token);

        let mut account = match account_found {
            Ok(account) => account,

            Err(err) => {
                println!("CAIU AQUI :{}", err);
                return Err(CustomError {
                    message: "User not found error".into(),
                    name: "UserNotFoundError".into(),
                    status: StatusCode::NOT_FOUND,
                });
            }
        };

        println!("{:?}", account);

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
