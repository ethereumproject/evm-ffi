#[cfg(not(feature = "std"))] use core::ffi::c_void;
#[cfg(feature = "std")] use std::ffi::c_void;
#[cfg(not(feature = "std"))] use core::marker::PhantomData;
#[cfg(feature = "std")] use std::marker::PhantomData;

use evm::{AccountPatch, DynamicPatch, Patch};
use common::c_gas;
use network::{MainnetAccountPatch, MordenAccountPatch, ETC_PRECOMPILEDS, BYZANTIUM_PRECOMPILEDS};
use CustomAccountPatch;

#[repr(C)]
pub enum precompiled_contract_set {
    ETC = 0,
    BYZANTIUM = 1,
}

#[repr(C)]
pub struct dynamic_patch_builder {
    /// Maximum contract size. 0 for unlimited.
    pub code_deposit_limit: usize,
    /// Limit of the call stack.
    pub callstack_limit: usize,
    /// Gas paid for extcode.
    pub gas_extcode: c_gas,
    /// Gas paid for BALANCE opcode.
    pub gas_balance: c_gas,
    /// Gas paid for SLOAD opcode.
    pub gas_sload: c_gas,
    /// Gas paid for SUICIDE opcode.
    pub gas_suicide: c_gas,
    /// Gas paid for SUICIDE opcode when it hits a new account.
    pub gas_suicide_new_account: c_gas,
    /// Gas paid for CALL opcode.
    pub gas_call: c_gas,
    /// Gas paid for EXP opcode for every byte.
    pub gas_expbyte: c_gas,
    /// Gas paid for a contract creation transaction.
    pub gas_transaction_create: c_gas,
    /// Whether to force code deposit even if it does not have enough
    /// gas.
    pub force_code_deposit: bool,
    /// Whether the EVM has DELEGATECALL opcode.
    pub has_delegate_call: bool,
    /// Whether the EVM has STATICCALL opcode.
    pub has_static_call: bool,
    /// Whether the EVM has REVERT opcode.
    pub has_revert: bool,
    /// Whether the EVM has RETURNDATASIZE and RETURNDATACOPY opcode.
    pub has_return_data: bool,
    /// Whether the EVM has SHL, SHR and SAR
    pub has_bitwise_shift: bool,
    /// Whether the EVM has EXTCODEHASH
    pub has_extcodehash: bool,
    /// Whether EVM should implement the EIP1283 gas metering scheme for SSTORE opcode
    pub has_reduced_sstore_gas_metering: bool,
    /// Whether to throw out of gas error when
    /// CALL/CALLCODE/DELEGATECALL requires more than maximum amount
    /// of gas.
    pub err_on_call_with_more_gas: bool,
    /// If true, only consume at maximum l64(after_gas) when
    /// CALL/CALLCODE/DELEGATECALL.
    pub call_create_l64_after_gas: bool,
    /// Maximum size of the memory, in bytes.
    /// NOTE: **NOT** runtime-configurable by block number
    pub memory_limit: usize,
}

pub type dynamic_patch_box = c_void;

fn dynamic_patch_new<A: AccountPatch>(builder: dynamic_patch_builder, contracts: precompiled_contract_set) -> *mut dynamic_patch_box {
    let mut patch = <DynamicPatch<A>>::default();
    patch.code_deposit_limit = if builder.code_deposit_limit == 0 { None } else { Some(builder.code_deposit_limit) };
    patch.callstack_limit = builder.callstack_limit;
    patch.gas_extcode = builder.gas_extcode.into();
    patch.gas_balance = builder.gas_balance.into();
    patch.gas_sload = builder.gas_sload.into();
    patch.gas_suicide = builder.gas_suicide.into();
    patch.gas_suicide_new_account = builder.gas_suicide_new_account.into();
    patch.gas_call = builder.gas_call.into();
    patch.gas_expbyte = builder.gas_expbyte.into();
    patch.gas_transaction_create = builder.gas_transaction_create.into();
    patch.force_code_deposit = builder.force_code_deposit;
    patch.has_delegate_call = builder.has_delegate_call;
    patch.has_static_call = builder.has_static_call;
    patch.has_revert = builder.has_revert;
    patch.has_return_data = builder.has_return_data;
    patch.has_bitwise_shift = builder.has_bitwise_shift;
    patch.has_extcodehash = builder.has_extcodehash;
    patch.has_reduced_sstore_gas_metering = builder.has_reduced_sstore_gas_metering;
    patch.err_on_call_with_more_gas = builder.err_on_call_with_more_gas;
    patch.call_create_l64_after_gas = builder.call_create_l64_after_gas;
    patch.memory_limit = builder.memory_limit;
    patch.precompileds = match contracts {
        precompiled_contract_set::ETC => &ETC_PRECOMPILEDS,
        precompiled_contract_set::BYZANTIUM => &ETC_PRECOMPILEDS,
    };
    
    Box::into_raw(Box::new(patch)) as *mut dynamic_patch_box
}

#[no_mangle]
pub extern "C" fn mainnet_dynamic_patch_new(builder: dynamic_patch_builder, contracts: precompiled_contract_set) -> *mut dynamic_patch_box {
    dynamic_patch_new::<MainnetAccountPatch>(builder, contracts)
}

#[no_mangle]
pub extern "C" fn morden_dynamic_patch_new(builder: dynamic_patch_builder, contracts: precompiled_contract_set) -> *mut dynamic_patch_box {
    dynamic_patch_new::<MordenAccountPatch>(builder, contracts)
}

#[no_mangle]
pub extern "C" fn custom_dynamic_patch_new(builder: dynamic_patch_builder, contracts: precompiled_contract_set) -> *mut dynamic_patch_box {
    dynamic_patch_new::<CustomAccountPatch>(builder, contracts)
}

#[no_mangle]
pub extern "C" fn dynamic_patch_free(patch: *mut dynamic_patch_box) {
    if patch.is_null() { return }
    // It's safe to erase type of AccountPatch as it's a size-less generic parameter
    unsafe { Box::from_raw(patch as *mut DynamicPatch<MainnetAccountPatch>); }
}
