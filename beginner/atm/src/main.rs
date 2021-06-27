// Reference: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
use std::io;

const ABC_BANK_CHARGES: f64 = 0.5;

struct Account {
    balance: f64,
}

impl Account {
    fn deposit(&mut self, amount: f64) -> bool {
        if amount <= 0.0 {
            println!("ERROR: Transaction not permitted. Deposit amount must be positive!");
            return false;
        }
        self.balance = self.balance + amount;
        true
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        
        if amount <= 0.0 {
            println!("ERROR: Transaction not permitted. Withdraw amount must be positive!");
            return false;
        }
        if self.balance < amount {
            println!("ERROR: Transaction not permitted. Insufficient balance!");
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

struct ABCBankAccount {
    account: Account,
    bank_charges: f64,
}

impl ABCBankAccount {
    fn deposit(&mut self, amount: u64) -> bool {
        let balance = self.account.balance + amount as f64;
        if balance > 2000.0 {
            println!("ERROR: Balance cannot be greater than 2000");
            return false;
        }
        self.account.deposit(amount as f64);
        true
    }

    fn withdraw(&mut self, amount: u64) -> bool {
        if amount % 5 != 0 {
            println!("ERROR: Withdraw amount must be a multiple of 5");
            return false
        }

        self.account.withdraw(amount as f64 + self.bank_charges);
        true
    }

    fn open_account(initial_balance: f64) -> ABCBankAccount {
        let mut account: Account = Account::open_account(0.0);
        
        if !account.deposit(initial_balance) {
            panic!("Invalid operation. Terminating.");
        } 
        
        ABCBankAccount {
            account: account,
            bank_charges: ABC_BANK_CHARGES,
        }
    }
}


fn main() {
    let mut pooja_account: ABCBankAccount;
    let mut _flag: bool;
    loop{
        println!("Please input the starting balance: ");
        let mut initial_balance = String::new();
        io::stdin()
            .read_line(&mut initial_balance)
            .expect("Failed to accept input");
        
        let initial_balance: f64 = match initial_balance.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("ERROR: Invalid input provided. Please provide a number less than equal to 2000$");
                continue;
                },
        };

        pooja_account = ABCBankAccount::open_account(initial_balance);
        break;
    }

    loop {
        println!("Please input the withdraw amount: ");
        let mut withdraw_amount = String::new();
        io::stdin()
            .read_line(&mut withdraw_amount)
            .expect("Failed to accept input");
        
        let withdraw_amount: u64 = match withdraw_amount.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("ERROR: Invalid input provided. Please provide a number.");
                continue;
                },
        };

        _flag = pooja_account.withdraw(withdraw_amount);
        if !_flag{
            continue;
        } 
        println!("Flag is {}", _flag);
        println!("Balance after transaction is {}", pooja_account.account.balance);
        
        println!("Do you want to continue (Y/N)?  ");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to accept input");
        
        if choice.trim() == "N" {
            println!("Thank you for transacting with us! Bye!");
            break;
        }
    }
}
