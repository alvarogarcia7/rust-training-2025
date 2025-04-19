use crate::TransferFundsError::{
    ReceiverNotExistsError, SenderNotEnoughBalance, SenderNotExistsError,
};

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
    pub(crate) fn merge_bank(&mut self, _other: &mut Bank) -> Bank {
        let mut merged_users: Vec<User> = vec![];
        // TODO: is there a function call chain to zip by a given property?
        // Instead of:
        // self.users.iter().zip(merged_users.iter_mut()).for_each(|(user, merged_user)| {})
        // Try:
        // https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.chunk_by
        //
        // self.users.chunk_by(|user| user.name);
        // Not working because the lambda needs to be a predicate
        //
        // Try
        // v.into_iter().map(|n| (n, f(n))).collect();, from https://www.reddit.com/r/rust/comments/18m24wb/how_to_convert_vec_to_hashmap/
        // let users_by_name = self.users.into_iter().map(|n| (n.name, n)).collect();
        // Compilation error = Cannot move 

        for user in &mut self.users {
            let maybe_overlapping_user = _other.users.iter_mut().find(|x| x.name == user.name);
            let mut balance = user.balance;
            if let Some(overlapping_user) = maybe_overlapping_user {
                balance += overlapping_user.balance;
                overlapping_user.balance = 0;
            }
            _other.users.retain(|x| x.name != user.name);
            merged_users.push(User::new(user.name.clone(), user.credit_line, balance));
        }

        for non_overlapping_user in &_other.users {
            merged_users.push(User::new(non_overlapping_user.name.clone(), non_overlapping_user.credit_line, non_overlapping_user.balance));
        }

        Bank {
            users: merged_users,
            name: self.name.clone(),
            credit_interest: self.credit_interest,
            debit_interest: self.debit_interest,
        }
    }
}

impl Bank {
    pub(crate) fn accrue_interest(&mut self) {
        for user in self.users.iter_mut() {
            let applicable_interest = match user.balance >= 0 {
                true => self.debit_interest,
                false => self.credit_interest,
            };
            user.balance += user.balance * applicable_interest as i64 / 100;
        }
    }
}

impl Bank {}

#[derive(Debug)]
struct Balance {
    value: i64,
}

impl Balance {
    pub(crate) fn new(value: i64) -> Self {
        Balance { value }
    }
}

impl PartialEq<Self> for Balance {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Bank {
    pub(crate) fn transfer_funds(
        &mut self,
        sender: String,
        receiver: String,
        amount: i64,
    ) -> Result<(), TransferFundsError> {
        let receiver_position = match self.index_of_user_by_username(receiver) {
            None => return Err(ReceiverNotExistsError),
            Some(index) => index,
        };

        let sender_position = match self.index_of_user_by_username(sender) {
            None => return Err(SenderNotExistsError),
            Some(x) => x,
        };

        if self.users[sender_position].balance < amount {
            return Err(SenderNotEnoughBalance);
        }

        self.users[sender_position].balance -= amount;
        self.users[receiver_position].balance += amount;

        Ok(())
    }

    fn index_of_user_by_username(&self, username: String) -> Option<usize> {
        self.users.iter().position(|u| u.name == username)
    }
}

struct BalanceSheet {
    liabilities: u64,
    assets: u64,
}

impl Bank {
    pub(crate) fn calc_balance(&self) -> BalanceSheet {
        let mut liabilities: i64 = 0;
        let mut assets: i64 = 0;

        for user in &self.users {
            if user.balance >= 0 {
                assets += user.balance;
            } else {
                liabilities += -user.balance;
            }
        }

        let liabilities: u64 = liabilities as u64;
        let assets: u64 = assets as u64;

        BalanceSheet {
            liabilities,
            assets,
        }
    }
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

enum TransferFundsError {
    SenderNotExistsError,
    ReceiverNotExistsError,
    SenderNotEnoughBalance,
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

    #[test]
    fn calculate_balance_sheet_with_negative_balance() {
        let user1 = User::new("Name Surname".to_string(), 0u64, -2i64);
        let user2 = User::new("Name Surname".to_string(), 0u64, 1i64);
        let bank = Bank::new(vec![user1, user2], "Bank Name".to_string(), 4u64, 1u64);

        let balance_sheet: BalanceSheet = bank.calc_balance();

        assert_eq!(balance_sheet.liabilities, 2u64);
        assert_eq!(balance_sheet.assets, 1u64);
    }
    #[test]
    fn calculate_balance_sheet_with_positive_balance() {
        let user1 = User::new("Name Surname".to_string(), 0u64, 2i64);
        let user2 = User::new("Name Surname".to_string(), 0u64, 1i64);
        let bank = Bank::new(vec![user1, user2], "Bank Name".to_string(), 4u64, 1u64);

        let balance_sheet: BalanceSheet = bank.calc_balance();

        assert_eq!(balance_sheet.liabilities, 0u64);
        assert_eq!(balance_sheet.assets, 3u64);
    }
    #[test]
    fn transfer_funds_happy_path() {
        let user1 = User::new("name1".to_string(), 0u64, 2i64);
        let user2 = User::new("name2".to_string(), 0u64, 1i64);
        let mut bank = Bank::new(vec![user1, user2], "Bank Name".to_string(), 4u64, 1u64);

        let result = bank.transfer_funds("name1".to_string(), "name2".to_string(), 2);

        assert!(result.is_ok());
        assert_eq!(bank.calc_balance().assets, 3u64);
        let bank_helper = BankHelper { bank: &bank };
        assert_eq!(bank_helper.balance_for("name1"), Balance::new(0i64));
        assert_eq!(bank_helper.balance_for("name2"), Balance::new(3i64));
    }
    #[test]
    fn transfer_funds_when_sender_does_not_exist() {
        let user2 = User::new("name2".to_string(), 0u64, 1i64);
        let mut bank = Bank::new(vec![user2], "Bank Name".to_string(), 4u64, 1u64);

        let result: Result<(), TransferFundsError> =
            bank.transfer_funds("nonexisting".to_string(), "name2".to_string(), 2);

        assert!(result.is_err());

        assert_eq!(bank.calc_balance().assets, 1u64);
        let bank_helper = BankHelper { bank: &bank };
        assert_eq!(bank_helper.balance_for("name2"), Balance::new(1i64));
    }

    #[test]
    fn transfer_funds_when_receiver_does_not_exist() {
        let user1 = User::new("name1".to_string(), 0u64, 1i64);
        let mut bank = Bank::new(vec![user1], "Bank Name".to_string(), 4u64, 1u64);

        let result: Result<(), TransferFundsError> =
            bank.transfer_funds("name1".to_string(), "nonexisting".to_string(), 2);

        assert!(result.is_err());

        assert_eq!(bank.calc_balance().assets, 1u64);
        let bank_helper = BankHelper { bank: &bank };
        assert_eq!(bank_helper.balance_for("name1"), Balance::new(1i64));
    }

    #[test]
    fn transfer_funds_when_not_enough_balance() {
        let user1 = User::new("name1".to_string(), 0u64, 2i64);
        let user2 = User::new("name2".to_string(), 0u64, 1i64);
        let mut bank = Bank::new(vec![user1, user2], "Bank Name".to_string(), 4u64, 1u64);

        let result = bank.transfer_funds("name1".to_string(), "name2".to_string(), 3);

        assert!(result.is_err());
        assert_eq!(bank.calc_balance().assets, 3u64);
        let bank_helper = BankHelper { bank: &bank };
        assert_eq!(bank_helper.balance_for("name1"), Balance::new(2i64));
        assert_eq!(bank_helper.balance_for("name2"), Balance::new(1i64));
    }

    #[test]
    fn accrue_interest() {
        let user1 = User::new("name1".to_string(), 0u64, -100i64);
        let user2 = User::new("name2".to_string(), 0u64, 100i64);
        let mut bank = Bank::new(vec![user1, user2], "Bank Name".to_string(), 4u64, 1u64);

        bank.accrue_interest();

        let bank_helper = BankHelper { bank: &bank };
        assert_eq!(bank_helper.balance_for("name1"), Balance::new(-104i64));
        assert_eq!(bank_helper.balance_for("name2"), Balance::new(101i64));
    }
    #[test]
    fn merge_bank() {
        let user1_1 = User::new("name1".to_string(), 0u64, 4i64);
        let mut bank1 = Bank::new(vec![user1_1], "Bank1".to_string(), 4u64, 1u64);
        let user1_2 = User::new("name1".to_string(), 0u64, 4i64);
        let user2 = User::new("name2".to_string(), 0u64, 2i64);
        let user3 = User::new("name3".to_string(), 0u64, 3i64);
        let mut bank2 = Bank::new(vec![user1_2, user2, user3], "Bank2".to_string(), 4u64, 1u64);

        let merged_bank = bank1.merge_bank(&mut bank2);

        let bank_helper = BankHelper { bank: &merged_bank };
        assert_eq!(bank_helper.balance_for("name1"), Balance::new(2 * 4i64));
        assert_eq!(bank_helper.balance_for("name2"), Balance::new(2i64));
        assert_eq!(bank_helper.balance_for("name3"), Balance::new(3i64));
    }

    struct BankHelper<'a> {
        bank: &'a Bank,
    }

    impl BankHelper<'_> {
        fn balance_for(&self, username: &str) -> Balance {
            (*self.bank).balance_of_user(username.to_string())
        }
    }

    impl Bank {
        pub(crate) fn balance_of_user(&self, user_name: String) -> Balance {
            Balance::new(self.users[self.index_of_user_by_username(user_name).unwrap()].balance)
        }
    }
}
