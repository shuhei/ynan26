use crate::{ErrorKind, Result};
use failure::ResultExt;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub ynab_token: String,
    pub ynab_budget_id: String,
    pub ynab_account_id: String,
    pub n26_username: String,
    pub n26_password: String,
}

fn read_env_var(name: &str) -> Result<String> {
    let value = env::var(name).context(ErrorKind::ReadEnvVar)?;
    Ok(value)
}

impl Config {
    // Read config from env variables
    pub fn from_env() -> Result<Config> {
        let ynab_token = read_env_var("YNAB_PERSONAL_TOKEN")?;
        let ynab_budget_id = read_env_var("YNAB_BUDGET_ID")?;
        let ynab_account_id = read_env_var("YNAB_ACCOUNT_ID")?;
        let n26_username = read_env_var("N26_USERNAME")?;
        let n26_password = read_env_var("N26_PASSWORD")?;

        let config = Config{
            ynab_token,
            ynab_budget_id,
            ynab_account_id,
            n26_username,
            n26_password,
        };
        Ok(config)
    }
}
