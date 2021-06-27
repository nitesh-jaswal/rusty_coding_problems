// Reference: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
use std::io;

struct Account {
    balance: f64,
}

impl Account {
    fn deposit(&mut self, amount: f64) -> bool {
        self.balance = self.balance + amount;
        true
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance < amount {
            return false;
        }

        self.balance = self.balance - amount;
        true
    }

    fn open_account(balance: f64) -> Account {
        Account {
            balance
        }
    }
}

struct ABCBank {
    account: Account,
    bank_charges: f64 = 0.5,
}

fn main() {
    println!("Hello, world!");
}
