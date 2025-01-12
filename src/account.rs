use crate::transaction::*;

#[derive(Debug, Clone)]
pub struct Account {
    id: usize,
    name: String,
}

#[derive(Debug, Clone)]
pub struct Ledger {
    accounts: Vec<Account>,
    transactions: Vec<Transaction>,
}
