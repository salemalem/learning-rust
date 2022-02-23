#[allow(dead_code)]
enum TransactionStatus {
    Confirmed,
    Unconfirmed
}

#[allow(dead_code)]
struct Transaction {
    sender: u128,
    receiver: u128,
    amount: u128,
}

fn main() {
    let transaction = Transaction {
        sender: 1,
        receiver: 2,
        amount: 1
    };
    println!("{}", transaction.amount);
}
