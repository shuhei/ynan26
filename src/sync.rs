use crate::{Result, Transaction, Ynab, N26};

pub struct Sync<'a> {
    pub ynab: &'a Ynab,
    pub n26: &'a N26,
}

// Find transactions in `x` that are not in `y`.
// TODO: Use generic instead Transaction
// TODO: Avoid O(m * n)
fn diff<'a>(x: &'a [Transaction], y: &'a [Transaction]) -> Vec<&'a Transaction> {
    x.iter()
        .filter(|t| {
            let found = y
                .iter()
                .find(|s| t.amount_in_cents == s.amount_in_cents && t.date == s.date);
            found.is_none()
        })
        .collect()
}

impl<'a> Sync<'a> {
    pub fn run(self: &Self) -> Result<()> {
        let mut ynab_transactions = self.ynab.get_transactions()?;
        ynab_transactions.reverse();

        let n26_transactions = self.n26.get_transactions()?;

        let only_n26 = diff(&n26_transactions, &ynab_transactions);
        for t in only_n26 {
            println!("Only N26: {:?}", t);
        }
        println!("\n");

        let only_ynab = diff(&ynab_transactions, &n26_transactions);
        for t in only_ynab {
            println!("Only YNAB: {:?}", t);
        }

        // TODO: Post new transactions to YNAB.

        Ok(())
    }
}
