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

struct Bank {
    users: Vec<User>,
    name: String,
    credit_interest: u64,
    debit_interest: u64,
}

impl Bank {
    pub(crate) fn new(
        users: Vec<User>,
        name: String,
        credit_interest: u64,
        debit_interest: u64,
    ) -> Self {
        Bank {
            users,
            name,
            credit_interest,
            debit_interest,
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

    #[test]
    fn bank_constructor_fields() {
        let user = User::new("Name Surname".to_string(), 4u64, -1i64);

        let bank = Bank::new(vec![user], "Bank Name".to_string(), 4u64, 1u64);

        assert_eq!(bank.users.len(), 1);
        assert_eq!(bank.name, "Bank Name".to_string());
        assert_eq!(bank.credit_interest, 4u64);
        assert_eq!(bank.debit_interest, 1u64);
    }
}
