use crate::{ErrorKind, Result};
use failure::ResultExt;
use oauth2::{AuthType, Config, Token};
use reqwest::header;
use serde::Deserialize;
use std::time::{Duration, SystemTime};

const API_URL: &str = "https://api.tech26.de";

#[derive(Debug)]
pub struct N26 {
    access_token: Token,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub amount: f32,
    #[serde(rename = "createdTS")]
    pub created_ts: i64,
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

    pub fn get_transactions(self: &Self) -> Result<Vec<Transaction>> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .context(ErrorKind::UnixTimestamp)?;
        let a_month_ago = now - Duration::new(60 * 60 * 24 * 30, 0);
        let from = a_month_ago.as_secs() * 1000;

        let limit = 100;
        let url = format!(
            "{}/api/smrt/transactions?from={}&limit={}",
            API_URL, from, limit
        );
        let authorization = format!("Bearer {}", self.access_token.access_token);

        let client = reqwest::Client::new();
        let mut res = client
            .get(&url)
            .header(header::AUTHORIZATION, authorization)
            .send()
            .context(ErrorKind::N26GetTransactions)?;

        let body = res.text().context(ErrorKind::N26GetTransactions)?;

        if !res.status().is_success() {
            let failure = ErrorKind::N26GetTransactionsFailure(res.status().as_u16(), body.clone());
            Err(failure)?;
        }

        let transactions: Vec<Transaction> =
            serde_json::from_str(&body).context(ErrorKind::N26GetTransactions)?;

        Ok(transactions)
    }
}
