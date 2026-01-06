use serde::{Deserialize, Serialize};
use weil_macros::{constructor, mutate, query, smart_contract, WeilType};

#[derive(Serialize, Deserialize, WeilType)]
pub struct CounterContractState {
    count: u64,
}

#[smart_contract]
impl CounterContractState {
    #[constructor]
    pub fn new() -> Result<Self, String> {
        Ok(Self { count: 0 })
    }

    #[query]
    pub fn get_count(&self) -> u64 {
        self.count
    }

    #[mutate]
    pub fn increment(&mut self) {
        self.count += 1;
    }
}
