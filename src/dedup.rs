use soroban_sdk::{contracttype, BytesN, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DedupKey {
    Action(BytesN<32>),
    Nonce(u64),
}

/// Sliding-window invocation deduplication and authorization replay guard.
///
/// Prevents cross-transaction replays and identical duplicate actions within
/// a configurable ledger window. Uses temporary storage with automated TTL extension
/// for short-lived action deduplication, and persistent storage with automated TTL extension
/// for replay authorization nonces so they survive ledger TTL eviction cycles.
pub struct DedupGuard;

impl DedupGuard {
    /// Default threshold ledgers to trigger persistent nonce TTL extension (~1 day at 5s/ledger).
    pub const NONCE_BUMP_THRESHOLD: u32 = 17_280;
    /// Default persistent nonce lifetime extension target (~7 days at 5s/ledger).
    pub const NONCE_EXTEND_TO: u32 = 120_960;

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
    /// Nonces are stored in persistent storage to eliminate potential replay on TTL eviction.
    /// Automatically extends the persistent entry's TTL if below NONCE_BUMP_THRESHOLD.
    pub fn verify_and_increment_nonce(env: &Env, user_nonce_key: u64, expected_nonce: u64) -> bool {
        Self::verify_and_increment_nonce_with_ttl(
            env,
            user_nonce_key,
            expected_nonce,
            Self::NONCE_BUMP_THRESHOLD,
            Self::NONCE_EXTEND_TO,
        )
    }

    /// Validates monotonically increasing sequential nonces with custom persistent TTL bump parameters.
    pub fn verify_and_increment_nonce_with_ttl(
        env: &Env,
        user_nonce_key: u64,
        expected_nonce: u64,
        threshold_ledgers: u32,
        extend_to_ledgers: u32,
    ) -> bool {
        let key = DedupKey::Nonce(user_nonce_key);
        let current_nonce: u64 = env.storage().persistent().get(&key).unwrap_or(0);

        if expected_nonce != current_nonce + 1 {
            return false;
        }

        env.storage().persistent().set(&key, &expected_nonce);
        env.storage()
            .persistent()
            .extend_ttl(&key, threshold_ledgers, extend_to_ledgers);
        true
    }
}
