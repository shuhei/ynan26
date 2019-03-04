use crate::{Result, Transaction, Ynab, N26};

pub struct Sync<'a> {
    pub ynab: &'a Ynab,
    pub n26: &'a N26,
}

// Find transactions in `x` that are not in `y`.
// TODO: Avoid O(m * n). Use Set?
fn diff<'a>(x: &'a [Transaction], y: &'a [Transaction]) -> Vec<&'a Transaction> {
    x.iter()
        .filter(|t| y.iter().find(|s| t.same(s)).is_none())
        .collect()
}

impl<'a> Sync<'a> {
    pub fn run(self: &Self) -> Result<()> {
        let mut ynab_transactions = self.ynab.get_transactions()?;
        // YNAB transactions are in the acending order while N26 transactions
        // are in the descending order. Here YNAB transactions are in the
        // descending order too (the newest one at the top).
        ynab_transactions.reverse();
        let n26_transactions = self.n26.get_transactions()?;

        println!("");

        let only_n26 = diff(&n26_transactions, &ynab_transactions);
        for t in &only_n26 {
            println!("+ Only N26: {:?}", t);
        }
        println!("");

        let only_ynab = diff(&ynab_transactions, &n26_transactions);
        for t in &only_ynab {
            println!("* Only YNAB: {:?}", t);
        }
        println!("");

        if only_n26.len() > 0 {
            println!("Posting {} transactions from N26 to YNAB", only_n26.len());
            self.ynab.post_transactions(only_n26.as_slice())?;
        } else {
            println!("No new transactions on N26");
        }

        Ok(())
    }
}
