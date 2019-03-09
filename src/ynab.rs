use crate::transaction;
use crate::{ErrorKind, Result};
use chrono::{Duration, Utc};
use failure::ResultExt;
use reqwest::{header, Method};
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://api.youneedabudget.com/v1";

// YNAB API client.
pub struct Ynab {
    pub personal_token: String,
    pub budget_id: String,
    pub account_id: String,
}

#[derive(Debug, Deserialize)]
struct TransactionsResponse {
    pub data: TransactionsWrapper,
}

#[derive(Debug, Deserialize)]
struct TransactionsWrapper {
    pub transactions: Vec<Transaction>,
}

// A transaction on YNAB. Not all fields are deserialized.
#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub id: String,

    // `amount: 1000` is 1 euro of deposit.
    // `amount: -1000` is 1 euro of spending.
    #[serde(rename = "amount")]
    pub amount_in_milliunits: i32,

    // Date in `YYYY-MM-DD` format. For example, `2019-03-01`.
    pub date: String,

    pub payee_name: Option<String>,

    pub import_id: Option<String>,
}

impl Into<transaction::Transaction> for Transaction {
    fn into(self: Self) -> transaction::Transaction {
        transaction::Transaction {
            id: self.id,
            amount_in_cents: self.amount_in_milliunits / 10,
            date: self.date,
            label: self.payee_name.unwrap_or("<not set>".to_string()),
            import_id: self.import_id,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SaveTransactionsRequest {
    pub data: SaveTransactionsWrapper,
}

#[derive(Debug, Serialize)]
pub struct SaveTransactionsWrapper {
    pub transactions: Vec<SaveTransaction>,
}

#[derive(Debug, Serialize)]
pub struct SaveTransaction {
    pub id: Option<String>,

    pub account_id: String,

    #[serde(rename = "amount")]
    pub amount_in_milliunits: i32,

    pub date: String,

    pub import_id: String,

    pub payee_name: Option<String>,
}

impl Ynab {
    // Get recent transactions in the budget and the account from YNAB.
    pub fn get_transactions(self: &Self) -> Result<Vec<transaction::Transaction>> {
        let a_month_ago = Utc::now() - Duration::days(30);

        // https://api.youneedabudget.com/v1#/Transactions/getTransactionsByAccount
        // If we use a database to store synchronization status, `last_knowledge_of_server` will be
        // useful.
        let since_date = a_month_ago.format("%Y-%m-%d");
        let url = format!(
            "{}/budgets/{}/accounts/{}/transactions?since_date={}",
            API_URL, self.budget_id, self.account_id, since_date
        );
        let authorization = format!("Bearer {}", self.personal_token);

        let client = reqwest::Client::new();
        let mut res = client
            .get(&url)
            .header(header::AUTHORIZATION, authorization)
            .send()
            .context(ErrorKind::YnabGetTransactions)?;

        let body = res.text().context(ErrorKind::YnabGetTransactions)?;

        if !res.status().is_success() {
            let http_error =
                ErrorKind::YnabGetTransactionsHttp(res.status().as_u16(), body.clone());
            Err(http_error)?;
        }

        let response: TransactionsResponse =
            serde_json::from_str(&body).context(ErrorKind::YnabGetTransactions)?;
        let transactions = response
            .data
            .transactions
            .into_iter()
            .map(|t| t.into())
            .collect();

        Ok(transactions)
    }

    // Post transactions into the budget and the account.
    pub fn post_transactions(
        &self,
        save_transactions: Vec<SaveTransaction>,
    ) -> Result<()> {
        self.send_save_transactions(
            Method::POST,
            ErrorKind::YnabPostTransactions,
            ErrorKind::YnabPostTransactionsHttp,
            save_transactions
        )
    }

    pub fn update_transactions(
        &self,
        save_transactions: Vec<SaveTransaction>,
    ) -> Result<()> {
        self.send_save_transactions(
            Method::PATCH,
            ErrorKind::YnabUpdateTransactions,
            ErrorKind::YnabUpdateTransactionsHttp,
            save_transactions
        )
    }

    fn send_save_transactions(
        &self,
        method: Method,
        error_kind: ErrorKind,
        error_kind_http: fn(u16, String) -> ErrorKind,
        save_transactions: Vec<SaveTransaction>,
    ) -> Result<()> {
        let wrapper = SaveTransactionsWrapper {
            transactions: save_transactions,
        };

        let url = format!("{}/budgets/{}/transactions", API_URL, self.budget_id);
        let authorization = format!("Bearer {}", self.personal_token);
        let req_body = serde_json::to_string(&wrapper).context(error_kind.clone())?;

        let client = reqwest::Client::new();
        let mut res = client
            .request(method, &url)
            .header(header::AUTHORIZATION, authorization)
            .header(header::ACCEPT, "application/json")
            .header(header::CONTENT_TYPE, "application/json")
            .body(req_body)
            .send()
            .context(error_kind.clone())?;

        if !res.status().is_success() {
            let res_body = res.text().context(error_kind.clone())?;
            let http_error = error_kind_http(res.status().as_u16(), res_body.clone());
            Err(http_error)?;
        }

        Ok(())
    }
}
