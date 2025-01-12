use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use std::fmt;

use crate::account::*;

#[derive(Debug, Copy, Clone)]
enum Direction {
    In,
    Out
}

#[derive(Debug, Clone)]
pub struct Split {
    account: Account,
    amount: Decimal,
    direction: Direction,
}

impl Split {

    pub fn new(account: Account, amount: Decimal, direction: Direction) -> Self {


    }

    pub fn reverse(&self) -> Self {


    }

    pub fn account(&self) -> &Account {
        &self.account

    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }
}

#[derive(Debug, Clone)]
pub struct Transaction {
    splits: Vec<Split>
}

impl Transaction {


    pub fn new(splits: Vec<Split>) -> Result<Self, TransactionError> {


    }

    pub fn new_simple(out_account: Account, in_account: Account, amount: Decimal) -> Self {


    }

    pub fn reverse(&self) -> Self {


    }

    pub fn scale(&self, x: f64) -> Self {

    }

    

}

#[derive(Debug, Clone)]
struct TransactionError;

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction values do not sum to zero")
    }
}

