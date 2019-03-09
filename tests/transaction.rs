extern crate ynan26;

#[cfg(test)]
mod transaction_tests {
    mod transaction {
        use ynan26::transaction::Transaction;

        #[test]
        fn partial_eq() {
            let a = Transaction {
                id: "abc".to_owned(),
                label: "foo".to_owned(),
                amount_in_cents: 123,
                date: "2019-02-10".to_owned(),
                import_id: None,
            };
            let b = Transaction {
                id: "bcd".to_owned(),
                label: "bar".to_owned(),
                amount_in_cents: 123,
                date: "2019-02-10".to_owned(),
                import_id: None,
            };
            assert!(a.same_amount_and_date(&b));
            assert!(b.same_amount_and_date(&a));
        }
    }
}
