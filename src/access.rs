use soroban_sdk::{contracterror, contracttype, Address, BytesN, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AccessError {
    Unauthorized = 301,
    TimelockNotElapsed = 302,
    TimelockExpired = 303,
    ActionAlreadyExecuted = 304,
}

pub struct Role;
impl Role {
    pub const ADMIN: u32 = 1 << 0;       // 1
    pub const OPERATOR: u32 = 1 << 1;    // 2
    pub const GUARDIAN: u32 = 1 << 2;    // 4
    pub const MINTER: u32 = 1 << 3;      // 8
    pub const LIQUIDATOR: u32 = 1 << 4;  // 16
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
enum AccessKey {
    Role(Address),
    TimelockAction(BytesN<32>),
    TimelockExecuted(BytesN<32>),
}

/// Bitflag-optimized Role-Based Access Control (RBAC) and Timelock Controller.
pub struct AccessControl;

impl AccessControl {
    /// Grants role mask to an address.
    pub fn grant_role(env: &Env, account: &Address, role_mask: u32) {
        let current_mask: u32 = env
            .storage()
            .instance()
            .get(&AccessKey::Role(account.clone()))
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&AccessKey::Role(account.clone()), &(current_mask | role_mask));
    }

    /// Revokes role mask from an address.
    pub fn revoke_role(env: &Env, account: &Address, role_mask: u32) {
        let current_mask: u32 = env
            .storage()
            .instance()
            .get(&AccessKey::Role(account.clone()))
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&AccessKey::Role(account.clone()), &(current_mask & !role_mask));
    }

    /// Checks if account possesses all required roles in mask.
    pub fn has_role(env: &Env, account: &Address, required_mask: u32) -> bool {
        let current_mask: u32 = env
            .storage()
            .instance()
            .get(&AccessKey::Role(account.clone()))
            .unwrap_or(0);
        (current_mask & required_mask) == required_mask
    }

    /// Asserts role possession and calls `account.require_auth()`.
    pub fn require_role(env: &Env, account: &Address, required_mask: u32) -> Result<(), AccessError> {
        account.require_auth();
        if !Self::has_role(env, account, required_mask) {
            return Err(AccessError::Unauthorized);
        }
        Ok(())
    }

    /// Schedules a timelocked administrative action.
    /// `execution_ledger = current_ledger + delay_ledgers`
    pub fn schedule_action(
        env: &Env,
        action_hash: &BytesN<32>,
        current_ledger: u32,
        delay_ledgers: u32,
    ) {
        let execution_ledger = current_ledger + delay_ledgers;
        env.storage().instance().set(
            &AccessKey::TimelockAction(action_hash.clone()),
            &execution_ledger,
        );
    }

    /// Verifies that timelock has matured and grace period has not expired.
    pub fn verify_timelock_and_consume(
        env: &Env,
        action_hash: &BytesN<32>,
        current_ledger: u32,
        grace_period_ledgers: u32,
    ) -> Result<(), AccessError> {
        if env
            .storage()
            .instance()
            .has(&AccessKey::TimelockExecuted(action_hash.clone()))
        {
            return Err(AccessError::ActionAlreadyExecuted);
        }

        let unlock_ledger: u32 = env
            .storage()
            .instance()
            .get(&AccessKey::TimelockAction(action_hash.clone()))
            .ok_or(AccessError::Unauthorized)?;

        if current_ledger < unlock_ledger {
            return Err(AccessError::TimelockNotElapsed);
        }

        if current_ledger > unlock_ledger + grace_period_ledgers {
            return Err(AccessError::TimelockExpired);
        }

        // Mark executed
        env.storage()
            .instance()
            .set(&AccessKey::TimelockExecuted(action_hash.clone()), &true);
        Ok(())
    }
}
