use crate::{ErrorKind, Result};
use failure::ResultExt;
use reqwest::header;
use serde::Deserialize;

const USER_AGENT: &str = "ynan26";

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

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub amount: i32,
    pub date: String,
}

impl Ynab {
    // Get recent transactions in the budget and the account from YNAB.
    pub fn get_transactions(self: &Self) -> Result<Vec<Transaction>> {
        // more useful.
        // TODO: Get the current date and subtract a month from it.
        let a_month_ago = "2019-02-03";

        // If we use a database to store synchronization status, `last_knowledge_of_server` will be
        let url = format!(
            "https://api.youneedabudget.com/v1/budgets/{}/accounts/{}/transactions?since_date={}",
            self.budget_id, self.account_id, a_month_ago
        );
        let authorization = format!("Bearer {}", self.personal_token);

        let client = reqwest::Client::new();
        let mut res = client
            .get(&url)
            .header(header::USER_AGENT, USER_AGENT)
            .header(header::AUTHORIZATION, authorization)
            .send()
            .context(ErrorKind::YnabGetTransactions)?;

        if !res.status().is_success() {
            Err(ErrorKind::YnabGetTransactions)?;
        }

        let body = res.text().context(ErrorKind::YnabGetTransactions)?;
        let response: TransactionsResponse =
            serde_json::from_str(&body).context(ErrorKind::YnabParseTransactions)?;

        Ok(response.data.transactions)
    }
}
