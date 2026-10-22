use soroban_sdk::{contracterror, contracttype, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ReentrancyError {
    ReentrancyDetected = 101,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
enum ReentrancyKey {
    Lock,
}

/// Zero-cost re-entrancy mutex for cross-contract invocations.
///
/// Uses temporary storage which automatically clears or can be manually dropped.
/// Prevents cross-contract re-entrancy attacks during external token transfers
/// and dynamic callbacks.
pub struct ReentrancyGuard;

impl ReentrancyGuard {
    /// Acquires the re-entrancy lock. Returns `Err(ReentrancyDetected)` if already entered.
    pub fn enter(env: &Env) -> Result<(), ReentrancyError> {
        if env.storage().temporary().has(&ReentrancyKey::Lock) {
            return Err(ReentrancyError::ReentrancyDetected);
        }
        env.storage().temporary().set(&ReentrancyKey::Lock, &true);
        Ok(())
    }

    /// Releases the re-entrancy lock.
    pub fn exit(env: &Env) {
        env.storage().temporary().remove(&ReentrancyKey::Lock);
    }

    /// Executes a closure safely wrapped within the re-entrancy mutex.
    pub fn with_guard<T, F: FnOnce() -> T>(env: &Env, f: F) -> Result<T, ReentrancyError> {
        Self::enter(env)?;
        let result = f();
        Self::exit(env);
        Ok(result)
    }
}
