mod types;

use types::BankAccount;
use types::Transaction;

fn main() {
    let mut account = BankAccount::new();

    // I deposited 100$
    account.make_transaction(Transaction::Deposit(100));

    // I withdrew 20$
    account.make_transaction(Transaction::Withdrawal(-20));

    // I withdrew another 30$
    account.make_transaction(Transaction::Withdrawal(-30));

    // What's my current balance?
    account.print_balance();
    account.count_withdrawals();

    let new_account = BankAccount::new();
    new_account.count_withdrawals();
}
