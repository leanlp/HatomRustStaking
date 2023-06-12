use multiversx_sc_scenario::{
    managed_address, managed_biguint, rust_biguint,
};
use multiversx_sc_scenario::ScenarioWorld;
use staking_contract::*;

#[test]
fn test_stake_successful() {
    let b_mock = ScenarioWorld::new();
    let owner_addr = "erd1qqqqqqqqqqqqqpgqkq3dsyrwrm3cxzpkghpky57cmp36ch5ch6psflcc57";
    let user_addr ="erd1qqqqqqqqqqqqqpgqkq3dsyrwrm3cxzpkghpky57cmp36ch5ch6psflcc57";
    let contract_obj = StakingPosition::new(&b_mock, &owner_addr); 
    
    // Assume the user has an EGLD balance of 1000
    const USER_BALANCE: u64 = 1000;

    b_mock.check_egld_balance(&user_addr, &rust_biguint!(USER_BALANCE));
    b_mock.check_egld_balance(contract_obj.address_ref(), &rust_biguint!(0));

    // stake full balance
    b_mock.execute_tx(&user_addr, &contract_obj, &rust_biguint!(USER_BALANCE), |sc| {
        sc.stake();
        let staking_pos = sc.staking_position(&managed_address!(&user_addr)).get();
        assert_eq!(staking_pos.stake_amount, managed_biguint!(USER_BALANCE));
    }).assert_ok();

    b_mock.check_egld_balance(&user_addr, &rust_biguint!(0));
    b_mock.check_egld_balance(contract_obj.address_ref(), &rust_biguint!(USER_BALANCE));
}

#[test]
fn test_stake_no_egld() {
    // similar to above, but try to stake without sending EGLD tokens, and assert that it fails
}