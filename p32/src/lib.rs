struct User {
    name: String,
    credit_line: u64,
    balance: i64,
}

impl User {
    pub(crate) fn new(name: String, credit_line: u64, balance: i64) -> Self {
        User {
            name,
            credit_line,
            balance,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_constructor_fields() {
        let user = User::new("Name Surname".to_string(), 4u64, -1i64);

        assert_eq!(user.name, "Name Surname".to_string());
        assert_eq!(user.credit_line, 4u64);
        assert_eq!(user.balance, -1i64);
    }
}
