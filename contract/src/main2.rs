#![no_std]
#![no_main]

extern crate alloc;

use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};

/// This logic is intended to be used as SESSION PAYMENT LOGIC
/// Alternate payment logic that allows payment from a purse other than the executing [Account]'s
/// main purse. A `Key::Uref` to the source purse must already exist in the executing context's
/// named keys under the name passed in as the `purse_name` argument.
#[no_mangle]
pub extern "C" fn call() {
    let amount = runtime::get_named_arg("amount");
    let my_purse = system::create_purse();
    runtime::put_key("mypurse", my_purse.into());
    system::transfer_from_purse_to_purse(account::get_main_purse(), my_purse, amount, None)
        .unwrap_or_revert();
}
