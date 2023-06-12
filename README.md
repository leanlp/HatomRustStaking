# Hatom Rust Staking

https://devnet-explorer.multiversx.com/accounts/erd1m84cdll9dqj8q5sx9nhm7snajs89u43ln27rrsnyyf9c4t8uh6ps3mmxz9


Challenge
Create a smart contract on the Elrond network that implements a staking
mechanism with the following features:
● Users can deposit their EGLD tokens into the contract to become a
staker.
● Assume rewards are distributed based on a global speed. For
example, 0.0003 EGLD are distributed per second among all users.
● Users earn rewards in proportion to their stake. These rewards are
continuously distributed (of course, they are kept at the staking smart
contract).
● Users can withdraw their staked EGLD or claim their rewards at any
time.
TECH CHALLENGE
Blockchain Developer
The document may not be shared with anyone not associated with Rather Labs. Copying or sharing it outside the provided link is prohibited.
Hint
Avoid using a loop to update the rewards for all stakers at any given
protocol interaction. The problem can be thought as having a share
amount and a share price: while the share amount an account has does
not change when someone else interacts with the protocol, the share price
does.
Total Rewards:
dR(t) = v(t) * dt
Account Rewards:
dr(t) = n(t) / N(t) * dR(t)
dr(t) = n(t) / N(t) * v(t) * dt
dr(t) = n(t) * dS(t)
In which:
dS(t) = 1 / N(t) * v(t) * dt
Is the Share price.

rewardRate
Sum of (reward rate * dt * 1e18 / total supply)

Si = amount staked by user. at time = i
Ti = total staked at time = i
R = reward rate per second, 

Si / Ti * R

//calculate reward per token 
r+= RewardRate/totalSupply *(current time - last update time)
// calculate reward earned by user
rewards[user] += stakedAmount[user] * (r - userRewardPerTokenPaid[user])
// update user reward per token paid
userRewardPerTokenPaid[user] = r
// update last update time
last update time = current time
// update staked amount
amountStaking +/- = amount () staling or withdraw
totalSupply +/- amount


The given equations describe how to calculate these values:

Total Rewards (dR(t)) is equal to the global speed (v(t)) times the time interval (dt).
The rewards for a specific account (dr(t)) is equal to their staked amount (n(t)) divided by the total staked amount (N(t)) times the total rewards.
This can be simplified to: The rewards for a specific account is equal to their staked amount times the change in the share price.
The change in the share price (dS(t)) is calculated as the global speed times the time interval, divided by the total staked amount.


deploy
    mxpy --verbose contract deploy --bytecode=output/staking-contract.wasm \
    --recall-nonce --pem=~/staking-contract/walletKey.pem \
    --gas-limit=30000000 \
    --send --outfile="deploy-devnet.interaction.json" --wait-result \
    --proxy=https://devnet-gateway.multiversx.com --chain=D  


stake
mxpy --verbose contract call  erd1qqqqqqqqqqqqqpgqpkmr924csxp3s8t4t8dgj0k9vfkc3zw0h6psrhn224\
    --proxy=https://devnet-gateway.multiversx.com --chain=D \
    --send --recall-nonce --pem=~/staking-contract/walletKey.pem \
    --gas-limit=20000000 \
    --value=100000000  \
    --function="stake"


unstake

    mxpy --verbose contract call  erd1qqqqqqqqqqqqqpgqpkmr924csxp3s8t4t8dgj0k9vfkc3zw0h6psrhn224\
    --proxy=https://devnet-gateway.multiversx.com --chain=D \
    --send --recall-nonce --pem=~/staking-contract/walletKey.pem \
    --gas-limit=10000000 \
    --arguments=100000000\
    --function="withdraw"


 mxpy --verbose contract call  erd1qqqqqqqqqqqqqpgqpkmr924csxp3s8t4t8dgj0k9vfkc3zw0h6psrhn224\
    --proxy=https://devnet-gateway.multiversx.com --chain=D \
    --send --recall-nonce --pem=~/staking-contract/walletKey.pem \
    --gas-limit=10000000 \
    --function="claim_rewards"


mxpy --verbose contract query  erd1qqqqqqqqqqqqqpgqpkmr924csxp3s8t4t8dgj0k9vfkc3zw0h6psrhn224\
    --proxy=https://devnet-gateway.multiversx.com \
    --function="getTotalStaked"

mxpy --verbose contract query  erd1qqqqqqqqqqqqqpgqpkmr924csxp3s8t4t8dgj0k9vfkc3zw0h6psrhn224\
    --proxy=https://devnet-gateway.multiversx.com \
    --function="getTotalRewards"

mxpy --verbose contract query  erd1qqqqqqqqqqqqqpgqpkmr924csxp3s8t4t8dgj0k9vfkc3zw0h6psrhn224\
    --proxy=https://devnet-gateway.multiversx.com \
    --function="getLastUpdateTimestamp"


  mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqnrszn5p7q7jj2zu7e7ngv2ugwvyrptj0h6psr98hwd\
    --proxy=https://devnet-gateway.multiversx.com \
    --function="getStakingPosition" \
    --arguments=erd1m84cdll9dqj8q5sx9nhm7snajs89u43ln27rrsnyyf9c4t8uh6ps3mmxz9

