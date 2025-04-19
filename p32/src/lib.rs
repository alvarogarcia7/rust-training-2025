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
    pub(crate) fn balance_of_user(&self, user_name: String) -> Balance {
        Balance::new(self.users[self.index_of_user_by_username(user_name).unwrap()].balance)
    }
}

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
            None => {
                return Err(SenderNotExistsError);
            }
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
        assert_eq!(
            bank.balance_of_user("name1".to_string()),
            Balance::new(0i64)
        );
        assert_eq!(
            bank.balance_of_user("name2".to_string()),
            Balance::new(3i64)
        );
    }
    #[test]
    fn transfer_funds_when_sender_does_not_exist() {
        let user2 = User::new("name2".to_string(), 0u64, 1i64);
        let mut bank = Bank::new(vec![user2], "Bank Name".to_string(), 4u64, 1u64);

        let result: Result<(), TransferFundsError> =
            bank.transfer_funds("nonexisting".to_string(), "name2".to_string(), 2);

        assert!(result.is_err());

        assert_eq!(bank.calc_balance().assets, 1u64);
        assert_eq!(
            bank.balance_of_user("name2".to_string()),
            Balance::new(1i64)
        );
    }

    #[test]
    fn transfer_funds_when_receiver_does_not_exist() {
        let user1 = User::new("name1".to_string(), 0u64, 1i64);
        let mut bank = Bank::new(vec![user1], "Bank Name".to_string(), 4u64, 1u64);

        let result: Result<(), TransferFundsError> =
            bank.transfer_funds("name1".to_string(), "nonexisting".to_string(), 2);

        assert!(result.is_err());

        assert_eq!(bank.calc_balance().assets, 1u64);
        assert_eq!(
            bank.balance_of_user("name1".to_string()),
            Balance::new(1i64)
        );
    }

    #[test]
    fn transfer_funds_when_not_enough_balance() {
        let user1 = User::new("name1".to_string(), 0u64, 2i64);
        let user2 = User::new("name2".to_string(), 0u64, 1i64);
        let mut bank = Bank::new(vec![user1, user2], "Bank Name".to_string(), 4u64, 1u64);

        let result = bank.transfer_funds("name1".to_string(), "name2".to_string(), 3);

        assert!(result.is_err());
        assert_eq!(bank.calc_balance().assets, 3u64);
        assert_eq!(
            bank.balance_of_user("name1".to_string()),
            Balance::new(2i64)
        );
        assert_eq!(
            bank.balance_of_user("name2".to_string()),
            Balance::new(1i64)
        );
    }
}
