extern crate dotenv;
extern crate failure;
extern crate ynan26;

use ynan26::{Config, Result, Sync, Ynab, N26};

fn main() -> Result<()> {
    // Read `.env` only on debug build
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

    let config = Config::from_env()?;

    let ynab = &Ynab {
        personal_token: config.ynab_token,
        budget_id: config.ynab_budget_id,
        account_id: config.ynab_account_id,
    };
    let n26 = &N26::authenticate(config.n26_username, config.n26_password)?;

    let sync = &Sync { n26, ynab };
    sync.run()?;

    Ok(())
}
