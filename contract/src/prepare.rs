#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use casper_contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{NamedKeys, Parameters},
    CLType, CLValue, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
};

/// This logic is intended to be used as SESSION PAYMENT LOGIC
/// Alternate payment logic that allows payment from a purse other than the executing [Account]'s
/// main purse. A `Key::Uref` to the source purse must already exist in the executing context's
/// named keys under the name passed in as the `purse_name` argument.
#[no_mangle]
pub extern "C" fn get_user_purse() {
    let purse = runtime::get_key("source_purse")
        .unwrap()
        .into_uref()
        .unwrap();

    runtime::ret(CLValue::from_t(purse).unwrap_or_revert());
}
#[no_mangle]
pub extern "C" fn call() {
    let amount = runtime::get_named_arg("amount");
    let my_purse = system::create_purse();

    system::transfer_from_purse_to_purse(account::get_main_purse(), my_purse, amount, None)
        .unwrap_or_revert();

    let mut named_keys = NamedKeys::new();
    named_keys.insert(String::from("source_purse"), my_purse.into());
    let mut entry_points = EntryPoints::new();
    let entry_point1 = EntryPoint::new(
        "get_user_purse",
        Parameters::new(),
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    entry_points.add_entry_point(entry_point1);

    let (contract_hash, _) = storage::new_contract(entry_points, Some(named_keys), None, None);
    runtime::put_key("mycontract", contract_hash.into());
}
