//! hello-world

#[macro_use]
extern crate pbc_contract_codegen;

use pbc_contract_common::address::Address;
use pbc_contract_common::context::{ContractContext};
use pbc_contract_common::info as log;


/// This is the state of the contract which is persisted on chain.
///
/// The #\[state\] macro generates serialization logic for the struct.
///
/// ### Fields:
///
///   * `name`: [`String`], the hello world greeting.
///
///
#[state]
struct ContractState {
    name: String,
}

impl ContractState {
    fn greet(&mut self, name: &str) {
        self.name = "Hello ".to_string() + name;
    }
}

#[init]
fn initialize(
    _ctx: ContractContext
) -> ContractState {

    let state = ContractState {
        name: "Hello World".to_string(),
    };

    state
}

#[action(shortname = 0x01)]
fn greet(
    _context: ContractContext,
    state: ContractState,
    name: String,
) -> ContractState {
    log(format!("Name is {}", name));
    assert!(!name.is_empty(), "name must not be empty");

    let mut new_state = state;
    new_state.greet(&name);

    new_state
}

