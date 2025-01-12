use crate::ledger::*;

pub struct Executor<I> {
    model: FinanceModel,
    ledger: Ledger,
    state: Option<I>,
}

pub struct FinanceModel {
    steps: Vec<Step>,



}

pub struct Step {

}


