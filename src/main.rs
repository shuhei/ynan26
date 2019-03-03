extern crate dotenv;
extern crate failure;
extern crate ynan26;

use ynan26::{Config, Result};

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

    Ok(())
}
