#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub id: String,
    pub amount_in_cents: i32,
    pub date: String,
    pub label: String,
}

impl Transaction {
    // We assume that two transactions are same if their amount and dates are same.
    pub fn same(self: &Self, other: &Self) -> bool {
        self.amount_in_cents == other.amount_in_cents && self.date == other.date
    }
}
