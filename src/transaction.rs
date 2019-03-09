#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Transaction {
    pub id: String,
    pub amount_in_cents: i32,
    pub date: String,
    pub label: String,
    pub import_id: Option<String>
}

impl Transaction {
    pub fn same_amount_and_date(&self, other: &Self) -> bool {
        self.amount_in_cents == other.amount_in_cents && self.date == other.date
    }

    pub fn imported_as(&self, other: &Self) -> bool {
        other.import_id.clone().map(|ii| ii == self.id).unwrap_or(false)
    }
}
