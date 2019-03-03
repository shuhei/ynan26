extern crate dotenv;
extern crate failure;
extern crate ynan26;

use ynan26::{Config, Result, N26, Sync, Ynab};

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

    let config = Config::from_env()?;
    println!("{:?}", config);

    let ynab = &Ynab{
        personal_token: config.ynab_token,
        budget_id: config.ynab_budget_id,
    };
    let n26 = &N26{
        username: config.n26_username,
        password: config.n26_password,
    };

    let sync = &Sync{
        n26,
        ynab,
    };
    sync.run()?;

    Ok(())
}
