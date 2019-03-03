extern crate dotenv;
extern crate failure;
extern crate ynan26;

use failure::ResultExt;
use std::env;
use ynan26::{ErrorKind, Result};

#[derive(Debug)]
struct Config {
    ynab_token: String,
    ynab_budget_id: String,
    n26_username: String,
    n26_password: String,
}

fn read_env_var(name: &str) -> Result<String> {
    let value = env::var(name).context(ErrorKind::ReadEnvVar)?;
    Ok(value)
}

// Read config from env variables
fn read_config() -> Result<Config> {
    let ynab_token = read_env_var("YNAB_PERSONAL_TOKEN")?;
    let ynab_budget_id = read_env_var("YNAB_BUDGET_ID")?;
    let n26_username = read_env_var("N26_USERNAME")?;
    let n26_password = read_env_var("N26_PASSWORD")?;

    let config = Config{
        ynab_token,
        ynab_budget_id,
        n26_username,
        n26_password,
    };
    Ok(config)
}

fn main() -> Result<()> {
    // Read .env only on debug build
    if cfg!(debug_assertions) {
        match dotenv::dotenv() {
            Err(e) => {
                println!("Failed to read from .env: {}", e);
            }
            Ok(path) => {
                println!("Read env variables from {:?}", path);
            }
        }
    }

    let config = read_config()?;
    println!("{:?}", config);

    Ok(())
}
