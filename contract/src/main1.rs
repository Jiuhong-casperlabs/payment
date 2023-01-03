#![no_std]
#![no_main]

extern crate alloc;

use casper_contract::contract_api::{runtime, storage};

/// This logic is intended to be used as SESSION PAYMENT LOGIC
/// Alternate payment logic that allows payment from a purse other than the executing [Account]'s
/// main purse. A `Key::Uref` to the source purse must already exist in the executing context's
/// named keys under the name passed in as the `purse_name` argument.
#[no_mangle]
pub extern "C" fn call() {
    runtime::put_key("hello", storage::new_uref("world").into());
}
