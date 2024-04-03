use diesel::prelude::*;
use diesel::result::Error;

use waiter_di::*;

use crate::accounts::domain::account;
use crate::accounts::features::sign_up::model::{Account, NewAccount};
use crate::shared::app_state::AppState;
use crate::shared::schema::accounts::dsl::*;

pub trait TSignUpRepository: Send {
    fn create(&self, account: &account::Account) -> Result<Account, Error>;
    fn update_one_by_id(&self, account: &account::Account) -> Result<bool, Error>;
}

#[component]
pub struct SignUpRepository {}

#[provides]
impl TSignUpRepository for SignUpRepository {
    fn create(&self, account: &account::Account) -> Result<Account, Error> {
        let mut conn = AppState::get_instance()
            .db_pool
            .get()
            .expect("Failed to get db connection");

        let new_account = NewAccount {
            name: &account.name,
            email: &account.email,
            password: &account.password,
        };

        let created = diesel::insert_into(accounts)
            .values(&new_account)
            .get_result::<Account>(&mut conn);

        match created {
            Ok(create_account) => Ok(create_account),
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
