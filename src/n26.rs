use crate::{ErrorKind, Result};
use failure::ResultExt;
use oauth2::{AuthType, Config, Token};

const API_URL: &str = "https://api.tech26.de";

#[derive(Debug)]
pub struct N26 {
    access_token: Token,
}

impl N26 {
    // Get an access token with a username and a password, and returns a N26 API client.
    // The way of authentication is based on https://github.com/guitmz/n26
    pub fn authenticate(username: String, password: String) -> Result<Self> {
        // The "password" grant flow doesn't use the authorize endpoint, and N26 doesn't seem to
        // expose it.
        let authorize_endpoint = format!("{}/noop", API_URL);
        let token_endpoint = format!("{}/oauth/token", API_URL);
        let mut config = Config::new("android", "secret", authorize_endpoint, token_endpoint);
        // OAuth2 has two ways of sending client ID and client secret. N26 uses basic auth.
        config = config.set_auth_type(AuthType::BasicAuth);

        let access_token = config
            .exchange_password(username, password)
            .context(ErrorKind::N26Authenticate)?;

        let client = N26 { access_token };
        Ok(client)
    }
}
