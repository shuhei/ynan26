use crate::transaction;
use crate::{ErrorKind, Result};
use chrono::{Duration, Utc};
use failure::ResultExt;
use reqwest::header;
use serde::Deserialize;

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
    // `amount: 1000` is 1 euro of deposit.
    // `amount: -1000` is 1 euro of spending.
    pub amount: i32,
    // Date in `YYYY-MM-DD` format. For example, `2019-03-01`.
    pub date: String,
    pub payee_name: String,
}

impl Into<transaction::Transaction> for Transaction {
    fn into(self: Self) -> transaction::Transaction {
        transaction::Transaction {
            amount_in_cents: self.amount / 10,
            date: self.date,
            label: self.payee_name,
        }
    }
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
            "https://api.youneedabudget.com/v1/budgets/{}/accounts/{}/transactions?since_date={}",
            self.budget_id, self.account_id, since_date
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
}
