
pub enum Transaction {
    Deposit(/*amount: */ i32),
    Withdrawal(/*amount: */ i32)
}
    
    
pub struct BankAccount {
    balance: i32,
    transactions: Vec<Transaction>,
}

impl BankAccount {
    pub fn new() -> BankAccount {
        return BankAccount {
            balance: 0,
            transactions: vec![]
        };
    }

    pub fn count_withdrawals(self) {
        // How many withdrawals have I done overall?
        let mut count = 0;
        for transaction in self.transactions {
            match transaction {
                Transaction::Deposit(amount) => {}
                Transaction::Withdrawal(amount) => {
                    count = count + 1;
                }
            }
        }
        println!("{count}");
    }

    pub fn make_transaction(&mut self, transaction: Transaction) {
        match transaction {
            Transaction::Deposit(amount) => {
                self.balance += amount;
            }
            Transaction::Withdrawal(amount) => {
                if amount * -1 > self.balance {
                    panic!("over withdrawal");
                }
                self.balance += amount;
            }
        }
        self.transactions.push(transaction);
    }

    pub fn print_balance(&self) {
        println!("{}", self.balance);
    }
}
