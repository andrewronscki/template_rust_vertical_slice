use diesel::prelude::*;
use diesel::result::Error;

use waiter_di::*;

use crate::accounts::domain::account;
use crate::accounts::features::sign_up::model::Account;
use crate::shared::app_state::AppState;
use crate::shared::schema::accounts::dsl::*;

pub trait TRefreshTokenRepository: Send {
    fn find_by_tokens(
        &self,
        user_access_token: &str,
        user_refresh_token: &str,
    ) -> Result<account::Account, Error>;
    fn update_one_by_id(&self, account: &account::Account) -> Result<bool, Error>;
}

#[component]
pub struct RefreshTokenRepository {}

#[provides]
impl TRefreshTokenRepository for RefreshTokenRepository {
    fn find_by_tokens(
        &self,
        user_access_token: &str,
        user_refresh_token: &str,
    ) -> Result<account::Account, Error> {
        let mut conn = AppState::get_instance()
            .db_pool
            .get()
            .expect("Failed to get db connection");

        let account_found = accounts
            .filter(
                access_token
                    .eq(user_access_token)
                    .and(refresh_token.eq(user_refresh_token)),
            )
            .first::<Account>(&mut conn);

        match account_found {
            Ok(account_found) => {
                print!("{:?}", account_found);
                let account = account::Account {
                    id: Some(account_found.id),
                    access_token: account_found.access_token.unwrap_or("".to_string()),
                    email: account_found.email,
                    name: account_found.name,
                    password: account_found.password,
                    refresh_token: account_found.refresh_token.unwrap_or("".to_string()),
                };
                Ok(account)
            }
            Err(e) => Err(e),
        }
    }

    fn update_one_by_id(&self, account: &account::Account) -> Result<bool, diesel::result::Error> {
        let mut conn = AppState::get_instance()
            .db_pool
            .get()
            .expect("Failed to get db connection");

        let user_id = account.id.unwrap();
        let target = accounts.filter(id.eq(user_id));
        let affected_rows = diesel::update(target)
            .set((
                access_token.eq(account.access_token.to_string()),
                refresh_token.eq(account.refresh_token.to_string()),
            ))
            .execute(&mut conn)?;

        Ok(affected_rows > 0)
    }
}
