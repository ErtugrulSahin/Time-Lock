use soroban_sdk::{contractimpl, Address, Env, Symbol, Map, contracttype, Vec, Duration, Timestamp};

pub struct TimeBoundToken;

#[contracttype]
pub struct ClaimableBalance {
    pub amount: i128,
    pub unlock_time: u64, // UNIX timestamp
    pub claimed: bool,
}

#[contractimpl]
impl TimeBoundToken {
    // Storage: Map<beneficiary, Vec<ClaimableBalance>>
    fn balances<'a>(env: &'a Env) -> Map<'a, Address, Vec<ClaimableBalance>> {
        env.storage().instance().get_map(Symbol::short("balances"))
    }

    // Create a claimable balance for an address, unlocks after `duration` seconds
    pub fn create_claimable(env: Env, beneficiary: Address, amount: i128, duration: u64) {
        let now: u64 = env.ledger().timestamp();
        let unlock_time = now + duration;
        let mut balances = Self::balances(&env);
        let mut user_balances = balances.get(beneficiary.clone()).unwrap_or(Vec::new(&env));
        user_balances.push_back(ClaimableBalance {
            amount,
            unlock_time,
            claimed: false,
        });
        balances.set(beneficiary, user_balances);
        env.storage().instance().set(Symbol::short("balances"), &balances);
    }

    // Claim available balances
    pub fn claim(env: Env, beneficiary: Address) -> i128 {
        let now: u64 = env.ledger().timestamp();
        let mut balances = Self::balances(&env);
        let mut user_balances = balances.get(beneficiary.clone()).unwrap_or(Vec::new(&env));
        let mut total_claimed = 0i128;
        for balance in user_balances.iter_mut() {
            if !balance.claimed && balance.unlock_time <= now {
                total_claimed += balance.amount;
                balance.claimed = true;
            }
        }
        balances.set(beneficiary, user_balances);
        env.storage().instance().set(Symbol::short("balances"), &balances);
        total_claimed
    }

    // (Opsiyonel) Kullanıcının claim edilebilir toplam bakiyesini görüntüle
    pub fn view_claimable(env: Env, beneficiary: Address) -> i128 {
        let now: u64 = env.ledger().timestamp();
        let balances = Self::balances(&env);
        let user_balances = balances.get(beneficiary).unwrap_or(Vec::new(&env));
        user_balances
            .iter()
            .filter(|b| !b.claimed && b.unlock_time <= now)
            .map(|b| b.amount)
            .sum()
    }
}
