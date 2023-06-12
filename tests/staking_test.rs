// use multiversx_sc::{codec::multi_types::OptionalValue, types::Address};
use multiversx_sc_scenario::{
    managed_address, managed_biguint, rust_biguint,
};
use multiversx_sc_scenario::whitebox::ContractObjWrapper;
use multiversx_sc_scenario::DebugApi;

use multiversx_sc_scenario::whitebox::BlockchainStateWrapper;
use multiversx_sc::api::ManagedTypeApi;
use multiversx_sc::types::BigUint;
use multiversx_sc::types::Address;
#[cfg(test)]
// use mockall::*;

use staking_contract::*;
#[test]
fn stake_withdraw_claim_rewards_test() {
    struct TestSetup<M: ManagedTypeApi, ContractObjBuilder> {
        staking_position: StakingPosition<M>,
        b_mock: BlockchainStateWrapper,
        pub contract_wrapper:
        // ContractObjWrapper<staking_contract::ContractObj<DebugApi>, ContractObjBuilder>,
        contract_wrapper::ContractWrapper,
    }
    let stake_amount = BigUint::from(0u32); 
    let last_update_timestamp = 1111111110;
    let setup = StakingPosition {
        stake_amount: stake_amount, 
        last_update_timestamp: last_update_timestamp,
    };
    // let setup = StakingPosition::new(stake_amount, last_update_timestamp);
    let owner_addr: &str= "erd1m84cdll9dqj8q5sx9nhm7snajs89u43ln27rrsnyyf9c4t8uh6ps3mmxz9";
    let user_addr:&str= "erd1qqqqqqqqqqqqqpgqkq3dsyrwrm3cxzpkghpky57cmp36ch5ch6psflcc57";
    // let owner_addr = Address::from("erd1m84cdll9dqj8q5sx9nhm7snajs89u43ln27rrsnyyf9c4t8uh6ps3mmxz9");
        // let user_addr = Address::from("erd1qqqqqqqqqqqqqpgqkq3dsyrwrm3cxzpkghpky57cmp36ch5ch6psflcc57");
    // let owner_addr = setup.owner_address.clone();
    // let user_addr = setup.user_address.clone();
    // let owner_addr = b_mock.create_user_account(&rust_zero);
    // let user_addr = b_mock.create_user_account(&rust_biguint!(USER_BALANCE));
    const USER_BALANCE: u64 = 1000;
    setup.b_mock.check_egld_balance(&user_addr, &rust_biguint!(USER_BALANCE));
    setup.b_mock.check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!(0));

    // stake full
    setup.b_mock.execute_tx(&user_addr, &setup.contract_wrapper, &rust_biguint!(USER_BALANCE), |sc| {
        sc.stake();
        let staking_pos = sc.staking_position(&managed_address!(&user_addr)).get();
        assert_eq!(staking_pos.stake_amount, managed_biguint!(USER_BALANCE));
    }).assert_ok();

    setup.b_mock.check_egld_balance(&user_addr, &rust_biguint!(0));
    setup.b_mock.check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!(USER_BALANCE));

    // withdraw partial
    // setup.b_mock.execute_tx(&user_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
    //     sc.withdraw(managed_biguint!(USER_BALANCE / 2));
    //     let staking_pos = sc.staking_position(&managed_address!(&user_addr)).get();
    //     assert_eq!(staking_pos.stake_amount, managed_biguint!(USER_BALANCE / 2));
    // }).assert_ok();

    // setup.b_mock.check_egld_balance(&user_addr, &rust_biguint!(USER_BALANCE / 2));
    // setup.b_mock.check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!(USER_BALANCE / 2));

    // // claim rewards
    // setup.b_mock.execute_tx(&user_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
    //     sc.claim_rewards();
    //     let staking_pos = sc.staking_position(&managed_address!(&user_addr)).get();
    //     // check if rewards are correctly claimed according to your rewards model
    // }).assert_ok();

    // // withdraw full
    // setup.b_mock.execute_tx(&user_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
    //     sc.withdraw(managed_biguint!(USER_BALANCE / 2));
    //     let staking_pos = sc.staking_position(&managed_address!(&user_addr)).get();
    //     assert_eq!(staking_pos.stake_amount, managed_biguint!(0));
    // }).assert_ok();

    // setup.b_mock.check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!(0));
}