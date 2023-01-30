#![no_std]
#![no_main]

extern crate alloc;

use casper_contract::contract_api::{runtime, storage};

#[no_mangle]
pub extern "C" fn call() {
    runtime::put_key("hello", storage::new_uref("world").into());
}
