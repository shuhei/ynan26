use crate::{Result, Transaction, SaveTransaction, Ynab, N26};
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Sync<'a> {
    pub ynab: &'a Ynab,
    pub n26: &'a N26,
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct UpdatedTransaction<'a> {
    pub source: &'a Transaction,
    pub destination: &'a Transaction,
}

impl<'a> Sync<'a> {
    pub fn run(self: &Self) -> Result<()> {
        let ynab_transactions = self.ynab.get_transactions()?;
        let n26_transactions = self.n26.get_transactions()?;

        println!("");

        let mut new_transactions = HashSet::new();
        let mut updated_transactions = HashSet::new();
        let mut only_ynab: HashSet<&Transaction> = HashSet::from_iter(&ynab_transactions);

        // Find new and updated transactions
        // O(m * n), but we won't have so many transactions anyway.
        for n in &n26_transactions {
            if let Some(y) = ynab_transactions.iter().find(|y| n.imported_as(y)) {
                // Already imported into YNAB
                only_ynab.remove(y);
                if !n.same_amount_and_date(y) {
                    // Amount or date were changed in N26
                    updated_transactions.insert(UpdatedTransaction{
                        source: n,
                        destination: y,
                    });
                }
            } else {
                // New transaction or manually input transaction
                if let Some(y) = ynab_transactions.iter().find(|y| n.same_amount_and_date(y)) {
                    // Most likely up-to-date manually input transaction
                    only_ynab.remove(y);
                } else {
                    // Most likely new transaction or outdated manually input transaction
                    new_transactions.insert(n);
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
            println!("Posting {} transactions from N26 to YNAB", new_transactions.len());
            let transactions = Vec::from_iter(new_transactions)
                .iter()
                .map(|t| SaveTransaction {
                    id: None,
                    account_id: self.ynab.account_id.to_owned(),
                    amount_in_milliunits: t.amount_in_cents * 10,
                    date: t.date.to_owned(),
                    // Using N26's transaction ID as is.
                    // I wanted to add a prefix, but YNAB allows only 36 characters in `import_id` and
                    // `id` from N26 is already 36 characters...
                    import_id: t.id.to_owned(),
                    payee_name: Some(t.label.to_owned()),
                })
                .collect();
            self.ynab.post_transactions(transactions)?;
        }

        if updated_transactions.len() > 0 {
            println!("Updating {} transactions on YNAB", updated_transactions.len());
            let transactions = Vec::from_iter(updated_transactions)
                .iter()
                .map(|UpdatedTransaction { source, destination }| SaveTransaction {
                    id: Some(destination.id.to_owned()),
                    account_id: self.ynab.account_id.to_owned(),
                    amount_in_milliunits: source.amount_in_cents * 10,
                    date: source.date.to_owned(),
                    import_id: source.id.to_owned(),
                    // Don't change manually edited payee name
                    payee_name: None,
                })
                .collect();
            self.ynab.update_transactions(transactions)?;
        }

        Ok(())
    }
}
