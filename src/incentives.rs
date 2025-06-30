//! Incentive mechanism module (simulated rewards)

static mut REWARD_BALANCE: u64 = 0;

pub fn start_incentives() {
    // Simulate earning a reward for running the node
    unsafe {
        REWARD_BALANCE += 10;
        println!(
            "Incentive mechanism started. Simulated reward: 10 tokens. Current balance: {}",
            REWARD_BALANCE
        );
    }
}

pub fn get_balance() -> u64 {
    unsafe { REWARD_BALANCE }
}

pub fn claim_reward(amount: u64) -> bool {
    unsafe {
        if REWARD_BALANCE >= amount {
            REWARD_BALANCE -= amount;
            println!(
                "Claimed {} tokens. New balance: {}",
                amount, REWARD_BALANCE
            );
            true
        } else {
            println!("Not enough balance to claim reward.");
            false
        }
    }
}
