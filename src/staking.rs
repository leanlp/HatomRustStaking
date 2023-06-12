#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const REWARD_RATE: u64 = 1/3000;  

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]

pub struct StakingPosition<M: ManagedTypeApi> {
    pub stake_amount: BigUint<M>,
    pub reward_per_token_paid: BigUint<M>,
    pub rewards: BigUint<M>,
}

#[multiversx_sc::contract]
pub trait StakingContract {
    #[init]   // self, take a struct inmutable
    fn init(&self) { 
            self.total_staked().set(BigUint::zero());
            self.total_rewards().set(BigUint::zero());
            self.reward_per_token_stored().set(BigUint::zero());
            self.last_update_timestamp().set(self.blockchain().get_block_timestamp());
        }

#[payable("EGLD")]

#[endpoint]
fn stake(&self) {
    let payment_amount = self.call_value().egld_value().clone_value();
    require!(payment_amount > 0, "Must pay more than 0");

    let caller = self.blockchain().get_caller();
    self.update_global_state();

    let mut staking_pos = if self.staking_position(&caller).is_empty() {
        StakingPosition {
            stake_amount: BigUint::zero(),
            reward_per_token_paid: BigUint::zero(),
            rewards: BigUint::zero(),
        }
    } else {
        self.staking_position(&caller).get()
    };

    // Calculate rewards and update reward_per_token_paid
    let rewards = (self.reward_per_token_stored().get() - staking_pos.reward_per_token_paid) * staking_pos.stake_amount.clone();
    staking_pos.stake_amount += rewards;  //add rewards to stake_amount of user
    staking_pos.reward_per_token_paid = self.reward_per_token_stored().get();

    // Update the staking position and total staked
    staking_pos.stake_amount += payment_amount.clone();
    self.total_staked().update(|val| *val += payment_amount);

}
  
    #[endpoint]
    fn withdraw(&self, amount: BigUint) {
        self.update_global_state();
    
        let caller = self.blockchain().get_caller();
        let mut staking_pos = self.staking_position(&caller).get();
    
        require!(staking_pos.stake_amount >= amount, "Not enough balance to withdraw");
    
        // Calculate rewards and update reward_per_token_paid
        let rewards = (self.reward_per_token_stored().get() - staking_pos.reward_per_token_paid.clone()) * staking_pos.stake_amount.clone();
        staking_pos.stake_amount += rewards.clone();
        staking_pos.reward_per_token_paid = self.reward_per_token_stored().get();
    
        // Deduct the withdrawn amount from the stake amount
        staking_pos.stake_amount -= amount.clone();
    
        // Update total rewards and staked
       
        self.total_staked().update(|val| *val -= &amount);
    
        // Store the updated staking position
        self.staking_position(&caller).set(&staking_pos);
    
        // Send withdrawn amount back to the user
        self.send().direct_egld(&caller, &amount);
    }
    #[endpoint]
    fn claim_rewards(&self) {
        self.update_global_state();
    
        let caller = self.blockchain().get_caller();
        let mut staking_pos = self.staking_position(&caller).get();
    
        let rewards = (self.reward_per_token_stored().get() - staking_pos.reward_per_token_paid.clone()) * staking_pos.stake_amount.clone();
        staking_pos.reward_per_token_paid = self.reward_per_token_stored().get();
    
        // Update rewards in staking position and reset
        staking_pos.rewards += &rewards;
        staking_pos.rewards = BigUint::zero();
    
         // Update total staked
        self.total_staked().update(|val| *val += rewards.clone());
        
      
        // Store the updated staking position
        self.staking_position(&caller).set(&staking_pos);
    
        // Send rewards to the user
        self.send().direct_egld(&caller, &rewards);
    }
   
    fn update_global_state(&self) {
        let current_timestamp = self.blockchain().get_block_timestamp();
        let elapsed_time = current_timestamp - self.last_update_timestamp().get();
        let total_staked = self.total_staked().get();

        if total_staked > 0 {
            let total_rewards = &total_staked * elapsed_time * REWARD_RATE;
            self.total_rewards().update(|val| *val += total_rewards.clone()); //*permit to change value
            self.reward_per_token_stored().update(|val| *val += total_rewards / total_staked);
        }

        self.last_update_timestamp().set(current_timestamp);
    }

    #[view(getRewardPerTokenStored)]
    #[storage_mapper("rewardPerTokenStored")]
    fn reward_per_token_stored(&self) -> SingleValueMapper<BigUint>;

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