use soroban_sdk::{Env, IntoVal, Val};

/// Storage management utilities for persistent and temporary Soroban data.
pub struct StorageManager;

impl StorageManager {
    /// Extends instance storage TTL if remaining lifetime drops below threshold.
    pub fn bump_instance(env: &Env, threshold_ledgers: u32, extend_to_ledgers: u32) {
        env.storage()
            .instance()
            .extend_ttl(threshold_ledgers, extend_to_ledgers);
    }

    /// Extends persistent storage TTL for a key if remaining lifetime drops below threshold.
    pub fn bump_persistent<K: IntoVal<Env, Val>>(
        env: &Env,
        key: &K,
        threshold_ledgers: u32,
        extend_to_ledgers: u32,
    ) {
        env.storage()
            .persistent()
            .extend_ttl(key, threshold_ledgers, extend_to_ledgers);
    }

    /// Extends temporary storage TTL for a key if remaining lifetime drops below threshold.
    pub fn bump_temporary<K: IntoVal<Env, Val>>(
        env: &Env,
        key: &K,
        threshold_ledgers: u32,
        extend_to_ledgers: u32,
    ) {
        env.storage()
            .temporary()
            .extend_ttl(key, threshold_ledgers, extend_to_ledgers);
    }
}
