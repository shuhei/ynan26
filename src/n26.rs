use crate::transaction;
use crate::{ErrorKind, Result};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use failure::ResultExt;
use oauth2::{AuthType, Config, Token};
use reqwest::header;
use serde::Deserialize;

const API_URL: &str = "https://api.tech26.de";

// N26 API client.
#[derive(Debug)]
pub struct N26 {
    access_token: Token,
}

// A transaction on N26. Not all fields are deserialized.
#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub id: String,

    // Amount in euros. Cents are represented as the fraction part.
    pub amount: f32,

    #[serde(rename = "visibleTS")]
    pub visible_ts: i64,

    #[serde(rename = "merchantName")]
    pub merchant_name: Option<String>,

    #[serde(rename = "partnerName")]
    pub partner_name: Option<String>,
}

impl Into<transaction::Transaction> for Transaction {
    fn into(self: Self) -> transaction::Transaction {
        let naive_time = NaiveDateTime::from_timestamp(self.visible_ts / 1000, 0);
        let date_time = DateTime::<Utc>::from_utc(naive_time, Utc);

        let cents = self.amount * 100.0;
        // A hack to avoid floating point rounding error.
        //   let n: f32 = -16.22;
        //   println!("{}", n * 100.0);
        //   // -1621.9999
        // TODO: Any better way?
        let abs_cents = (cents.abs() + 0.001) as i32;
        let amount_in_cents = if cents >= 0.0 {
            abs_cents
        } else {
            -abs_cents
        };

        transaction::Transaction {
            id: self.id,
            amount_in_cents,
            date: date_time.format("%Y-%m-%d").to_string(),
            label: self.merchant_name.or(self.partner_name).unwrap_or("<not set>".to_string()),
            import_id: None,
        }
    }
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

    // Get transactions of the last 30 days.
    pub fn get_transactions(self: &Self) -> Result<Vec<transaction::Transaction>> {
        let now = Utc::now();
        let a_month_ago = now - Duration::days(30);

        // `from` and `to` have to be used together.
        let from = a_month_ago.timestamp_millis();
        let to = now.timestamp_millis();
        let limit = 100;
        let url = format!(
            "{}/api/smrt/transactions?from={}&to={}&limit={}",
            API_URL, from, to, limit
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
            let http_error = ErrorKind::N26GetTransactionsHttp(res.status().as_u16(), body.clone());
            Err(http_error)?;
        }

        let n26_transactions: Vec<Transaction> =
            serde_json::from_str(&body).context(ErrorKind::N26GetTransactions)?;
        let transactions = n26_transactions.into_iter().map(|t| t.into()).collect();

        Ok(transactions)
    }
}
