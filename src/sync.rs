use crate::n26::{N26Transaction, N26};
use crate::ynab::{SaveTransaction, Ynab, YnabTransaction};
use crate::{Result, Transaction};
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Sync<'a> {
    pub ynab: &'a Ynab,
    pub n26: &'a N26,
}

#[derive(Debug)]
pub struct UpdatedTransaction<'a> {
    pub n26: &'a N26Transaction,
    pub ynab: &'a YnabTransaction,
}

impl<'a> Sync<'a> {
    pub fn run(self: &Self) -> Result<()> {
        let ynab_transactions = self.ynab.get_transactions()?;
        let n26_transactions = self.n26.get_transactions()?;

        println!("");

        let mut new_transactions = vec![];
        let mut updated_transactions = vec![];
        let mut only_ynab: HashSet<&YnabTransaction> = HashSet::from_iter(&ynab_transactions);

        // Find new and updated transactions
        // O(m * n), but we won't have so many transactions anyway.
        for n in &n26_transactions {
            if let Some(y) = ynab_transactions
                .iter()
                .find(|y| y.import_id.clone().map(|ii| ii == n.id).unwrap_or(false))
            {
                // Already imported into YNAB
                only_ynab.remove(y);
                if !n.same_amount_and_date(y) {
                    // Amount or date were changed in N26
                    updated_transactions.push(UpdatedTransaction { n26: n, ynab: y });
                }
            } else {
                // New transaction or manually input transaction
                if let Some(y) = ynab_transactions.iter().find(|y| n.same_amount_and_date(y.clone())) {
                    // Most likely up-to-date manually input transaction
                    only_ynab.remove(y);
                } else {
                    // Most likely new transaction or outdated manually input transaction
                    new_transactions.push(n);
                }
            }
        }

        for t in &new_transactions {
            println!("+ new transaction in N26: {:?}", t);
        }
        println!("");

        for t in &updated_transactions {
            println!("* updated transaction: {:?}", t);
        }
        println!("");

        for t in &only_ynab {
            println!("- only in YNAB: {:?}", t);
        }
        println!("");

        if new_transactions.len() > 0 {
            println!(
                "Posting {} transactions from N26 to YNAB",
                new_transactions.len()
            );
            let transactions = new_transactions
                .iter()
                .map(|n26| SaveTransaction {
                    id: None,
                    account_id: self.ynab.account_id.to_owned(),
                    amount_in_milliunits: n26.amount_in_cents() * 10,
                    date: n26.date().to_owned(),
                    // Using N26's transaction ID as is.
                    // I wanted to add a prefix, but YNAB allows only 36 characters in `import_id` and
                    // `id` from N26 is already 36 characters...
                    import_id: n26.id.to_owned(),
                    payee_name: n26.payee_name(),
                })
                .collect();
            self.ynab.post_transactions(transactions)?;
        }

        if updated_transactions.len() > 0 {
            println!(
                "Updating {} transactions on YNAB",
                updated_transactions.len()
            );
            let transactions = updated_transactions
                .iter()
                .map(|UpdatedTransaction { n26, ynab }| SaveTransaction {
                    id: Some(ynab.id.to_owned()),
                    account_id: self.ynab.account_id.to_owned(),
                    amount_in_milliunits: n26.amount_in_cents() * 10,
                    date: n26.date().to_owned(),
                    import_id: ynab.id.to_owned(),
                    // Don't change manually edited payee name
                    payee_name: ynab.payee_name().or(n26.payee_name()),
                })
                .collect();
            self.ynab.update_transactions(transactions)?;
        }

        Ok(())
    }
}
