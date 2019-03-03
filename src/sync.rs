use crate::{Result, Ynab, N26};

pub struct Sync<'a> {
    pub ynab: &'a Ynab,
    pub n26: &'a N26,
}

impl<'a> Sync<'a> {
    pub fn run(self: &Self) -> Result<()> {
        let ynab_transactions = self.ynab.get_transactions()?;
        println!(
            "YNAB transactions:\n---------------\n{:?}\n---------------",
            ynab_transactions
        );

        // TODO: Get transactions from N26.
        // TODO: Compare transactions.
        // TODO: Post new transactions to YNAB.

        Ok(())
    }
}
