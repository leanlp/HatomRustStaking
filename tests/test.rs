use multiversx_sc_scenario::{
    managed_address, managed_biguint, rust_biguint, whitebox::*, DebugApi,
};
use multiversx_sc_scenario::BlockchainMock;
use staking_contract::*;

#[test]
fn main() {
    let b_mock = BlockchainMock::new();

    // Initialize your contract here
    // let contract_obj = StakingContract::new(&b_mock);

    // let owner_addr = managed_address!("owner");
    // let user_addr = managed_address!("user");

    // const USER_BALANCE: u64 = 1000;

    // b_mock.check_egld_balance(&user_addr, &rust_biguint!(USER_BALANCE));
    // b_mock.check_egld_balance(contract_obj.address_ref(), &rust_biguint!(0));

    // // stake full
    // b_mock.execute_tx(&user_addr, &contract_obj, &rust_biguint!(USER_BALANCE), |sc| {
    //     sc.stake();
    //     let staking_pos = sc.staking_position(&managed_address!(&user_addr)).get();
    //     assert_eq!(staking_pos.stake_amount, managed_biguint!(USER_BALANCE));
    // }).assert_ok();

    // b_mock.check_egld_balance(&user_addr, &rust_biguint!(0));
    // b_mock.check_egld_balance(contract_obj.address_ref(), &rust_biguint!(USER_BALANCE));
}

#[test]
fn test_init() {
    let mut b_mock = BlockchainMock::new();
    let owner_addr = managed_address!("owner");

    let contract_obj = StakingContractImpl::new(&b_mock, &owner_addr);

    // Call init function
    b_mock.execute_tx(&owner_addr, &contract_obj, &rust_biguint!(0), |sc| {
        sc.init();
    }).assert_ok();

    // Check if the total_staked, total_rewards and last_update_timestamp are correctly set
    assert_eq!(contract_obj.total_staked().get(), managed_biguint!(0));
    assert_eq!(contract_obj.total_rewards().get(), managed_biguint!(0));
    assert_eq!(contract_obj.last_update_timestamp().get(), b_mock.get_block_timestamp());
}