use crate::{N26, Ynab, Result};

pub struct Sync<'a> {
    pub ynab: &'a Ynab,
    pub n26: &'a N26,
}

impl<'a> Sync<'a> {
    pub fn run(self: &Self) -> Result<()> {
        Ok(())
    }
}
