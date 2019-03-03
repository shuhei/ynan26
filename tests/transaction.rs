extern crate ynan26;

#[cfg(test)]
mod transaction_tests {
    mod transaction {
        use ynan26::transaction::Transaction;

        #[test]
        fn partial_eq() {
            let a = Transaction {
                amount_in_cents: 123,
                date: "2019-02-10".to_owned(),
            };
            let b = Transaction {
                amount_in_cents: 123,
                date: "2019-02-10".to_owned(),
            };
            assert_eq!(a, b);
        }
    }
}
