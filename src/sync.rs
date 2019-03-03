use crate::{Result, Ynab, N26, Transaction};

pub struct Sync<'a> {
    pub ynab: &'a Ynab,
    pub n26: &'a N26,
}

// Find transactions in `x` that are not in `y`.
// TODO: Use generic instead Transaction
// TODO: Avoid O(m * n)
fn diff<'a>(x: &'a [Transaction], y: &'a [Transaction]) -> Vec<&'a Transaction> {
    x.iter().filter(|t| !y.contains(t)).collect()
}

impl<'a> Sync<'a> {
    pub fn run(self: &Self) -> Result<()> {
        let mut ynab_transactions = self.ynab.get_transactions()?;
        ynab_transactions.reverse();
        println!(
            "YNAB transactions:\n---------------\n{:?}\n---------------",
            ynab_transactions
        );

        let n26_transactions = self.n26.get_transactions()?;
        println!(
            "N26 transactions:\n---------------\n{:?}\n---------------",
            n26_transactions
        );

        let only_n26 = diff(&n26_transactions, &ynab_transactions);
        for t in only_n26 {
            println!("Only N26: {:?}", t);
        }

        let only_ynab = diff(&ynab_transactions, &n26_transactions);
        for t in only_ynab {
            println!("Only YNAB: {:?}", t);
        }

        // TODO: Post new transactions to YNAB.

        Ok(())
    }
}
