use soroban_sdk::{contracttype, BytesN, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DedupKey {
    Action(BytesN<32>),
    Nonce(u64),
}

/// Sliding-window invocation deduplication guard.
///
/// Prevents cross-transaction replays and identical duplicate actions within
/// a configurable ledger window. Uses temporary storage with automated TTL extension
/// to ensure zero persistent storage pollution.
pub struct DedupGuard;

impl DedupGuard {
    /// Checks whether an action identifier has already been executed in the current window.
    /// Returns `true` if already executed (duplicate), `false` if fresh.
    pub fn is_duplicate(env: &Env, action_hash: &BytesN<32>) -> bool {
        let key = DedupKey::Action(action_hash.clone());
        env.storage().temporary().has(&key)
    }

    /// Records an action hash and sets its expiration window in ledgers.
    /// If the action has already been executed, returns `false` (suppressed).
    /// If newly registered, records the action, extends TTL, and returns `true`.
    pub fn record_action(env: &Env, action_hash: &BytesN<32>, window_ledgers: u32) -> bool {
        let key = DedupKey::Action(action_hash.clone());
        if env.storage().temporary().has(&key) {
            return false;
        }

        env.storage().temporary().set(&key, &true);
        env.storage()
            .temporary()
            .extend_ttl(&key, window_ledgers, window_ledgers);
        true
    }

    /// Validates monotonically increasing sequential nonces for off-chain signed permits.
    pub fn verify_and_increment_nonce(env: &Env, user_nonce_key: u64, expected_nonce: u64) -> bool {
        let key = DedupKey::Nonce(user_nonce_key);
        let current_nonce: u64 = env.storage().temporary().get(&key).unwrap_or(0);

        if expected_nonce != current_nonce + 1 {
            return false;
        }

        env.storage().temporary().set(&key, &expected_nonce);
        true
    }
}
