use soroban_sdk::Env;

/// Storage management utilities for persistent and temporary Soroban data.
pub struct StorageManager;

impl StorageManager {
    /// Extends instance storage TTL if remaining lifetime drops below threshold.
    pub fn bump_instance(env: &Env, threshold_ledgers: u32, extend_to_ledgers: u32) {
        env.storage()
            .instance()
            .extend_ttl(threshold_ledgers, extend_to_ledgers);
    }
}
