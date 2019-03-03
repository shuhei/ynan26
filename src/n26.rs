use crate::{Result};

pub struct N26 {
    pub username: String,
    pub password: String,
}

impl N26 {
    pub fn authenticate<'a>(username: String, password: String) -> Result<Self> {
        // TODO: Get an access token with OAuth and create a client with it.
        let client = N26 {
            username: username,
            password: password,
        };
        Ok(client)
    }
}
