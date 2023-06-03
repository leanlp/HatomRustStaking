#![no_std]
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const REWARD_RATE: u64 = 3000;  // Rewards per second in 1e18 format

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]
pub struct StakingPosition<M: ManagedTypeApi> {
    pub stake_amount: BigUint<M>,
    pub last_update_timestamp: u64,
}

#[multiversx_sc::contract]
pub trait StakingContract {
    #[init]
    fn init(&self) {
        self.total_staked().set(BigUint::zero());
        self.total_rewards().set(BigUint::zero());
        self.last_update_timestamp().set(self.blockchain().get_block_timestamp());
    }

   
#[payable("EGLD")]
#[endpoint]
fn stake(&self) {
    let payment_amount = self.call_value().egld_value().clone_value();
    let payment_amount_clone = payment_amount.clone();
    require!(payment_amount > 0, "Must pay more than 0");

    self.update_global_state();

    let caller = self.blockchain().get_caller();

    let mut staking_pos: StakingPosition<Self::Api>;
    if self.staking_position(&caller).is_empty() {
        staking_pos = StakingPosition {
            stake_amount: BigUint::zero(),
            last_update_timestamp: self.blockchain().get_block_timestamp(),
        };
    } else {
        staking_pos = self.staking_position(&caller).get();
    }

    let current_timestamp = self.blockchain().get_block_timestamp();
    let elapsed_time = current_timestamp - staking_pos.last_update_timestamp;
    let rewards = &staking_pos.stake_amount * elapsed_time / REWARD_RATE;
    let rewards_clone = rewards.clone();
    staking_pos.last_update_timestamp = current_timestamp;

    staking_pos.stake_amount += &payment_amount_clone;
    self.total_staked().update(|val| *val += &payment_amount_clone.clone());

    self.total_rewards().update(|val| *val += &rewards.clone());
    self.send().direct_egld(&caller, &rewards_clone);

    self.staking_position(&caller).set(&staking_pos);
}
  
    
    #[endpoint]
    fn withdraw(&self, amount: BigUint) {
        self.update_global_state();

        let caller = self.blockchain().get_caller();
        let mut staking_pos = self.staking_position(&caller).get();

        require!(staking_pos.stake_amount >= amount, "Not enough balance to withdraw");

        let current_timestamp = self.blockchain().get_block_timestamp();
        let elapsed_time = current_timestamp - staking_pos.last_update_timestamp;
        let rewards = &staking_pos.stake_amount * elapsed_time / REWARD_RATE;
        staking_pos.last_update_timestamp = current_timestamp;

        staking_pos.stake_amount -= &amount;

        self.staking_position(&caller).set(&staking_pos);
        self.total_staked().update(|val| *val -= &amount);
        self.total_rewards().update(|val| *val += rewards);

        self.send().direct_egld(&caller, &amount);
    }

    #[endpoint]
    fn claim_rewards(&self) {
        self.update_global_state();
    
        let caller = self.blockchain().get_caller();
        let mut staking_pos = self.staking_position(&caller).get();
    
        let current_timestamp = self.blockchain().get_block_timestamp();
        let elapsed_time = current_timestamp - staking_pos.last_update_timestamp;
        let rewards = &staking_pos.stake_amount * elapsed_time / REWARD_RATE;
        let rewards_clone = rewards.clone();
        staking_pos.last_update_timestamp = current_timestamp;
    
        self.staking_position(&caller).set(&staking_pos);
        self.total_rewards().update(|val| *val += rewards);
    
        self.send().direct_egld(&caller, &rewards_clone);
    }

    fn update_global_state(&self) {
        let current_timestamp = self.blockchain().get_block_timestamp();
        let elapsed_time = current_timestamp - self.last_update_timestamp().get();
        let total_rewards = &self.total_staked().get() * elapsed_time / REWARD_RATE;
        
        self.total_rewards().update(|val| *val += total_rewards);
        self.last_update_timestamp().set(current_timestamp);
    }

    #[view(getStakingPosition)]
    #[storage_mapper("stakingPosition")]
    fn staking_position(&self, addr: &ManagedAddress) -> SingleValueMapper<StakingPosition<Self::Api>>;

    #[view(getTotalStaked)]
    #[storage_mapper("totalStaked")]
    fn total_staked(&self) -> SingleValueMapper<BigUint>;

    #[view(getTotalRewards)]
    #[storage_mapper("totalRewards")]
    fn total_rewards(&self) -> SingleValueMapper<BigUint>;

    #[view(getLastUpdateTimestamp)]
    #[storage_mapper("lastUpdateTimestamp")]
    fn last_update_timestamp(&self) -> SingleValueMapper<u64>;
}
