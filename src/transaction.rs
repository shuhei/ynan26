pub trait Transaction {
    fn amount_in_cents(&self) -> i32;
    fn date(&self) -> String;
    fn payee_name(&self) -> Option<String>;

    fn same_amount_and_date(&self, other: &Transaction) -> bool {
        self.amount_in_cents() == other.amount_in_cents() && self.date() == other.date()
    }
}
