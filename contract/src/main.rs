#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;

use casper_contract::{
    contract_api::{runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ContractHash, RuntimeArgs, URef, U512};

const GET_PAYMENT_PURSE: &str = "get_payment_purse";
const SET_REFUND_PURSE: &str = "set_refund_purse";

const ARG_AMOUNT: &str = "amount";
const ARG_PURSE: &str = "purse";

#[no_mangle]
pub extern "C" fn call() {
    let contract_hash_str: String = runtime::get_named_arg("pursecontract");
    let contract_hash = ContractHash::from_formatted_str(&contract_hash_str).unwrap();
    let purse_uref: URef =
        runtime::call_contract(contract_hash, "get_user_purse", runtime_args! {});

    // amount to transfer from named purse to payment purse
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);

    // handle payment contract
    let handle_payment_hash = system::get_handle_payment();

    // set refund purse to source purse
    {
        let contract_hash = handle_payment_hash;
        let args = runtime_args! {
            ARG_PURSE => purse_uref,
        };
        runtime::call_contract::<()>(contract_hash, SET_REFUND_PURSE, args);
    }

    // get payment purse for current execution
    let payment_purse: URef = runtime::call_contract(
        handle_payment_hash,
        GET_PAYMENT_PURSE,
        RuntimeArgs::default(),
    );

    // transfer amount from named purse to payment purse, which will be used to pay for execution
    system::transfer_from_purse_to_purse(purse_uref, payment_purse, amount, None)
        .unwrap_or_revert();
}
