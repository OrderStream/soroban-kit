use soroban_sdk::{contracterror, contracttype, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum CircuitBreakerError {
    ProtocolHalted = 201,
    OperationNotPermittedInDegradedMode = 202,
    InvalidStateTransition = 203,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ProtocolState {
    Operational = 0,
    Degraded = 1,
    Halted = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
enum PauseKey {
    State,
    IncidentCount,
    IncidentWindowLedger,
}

/// 3-Tier Multi-State Emergency Circuit Breaker.
///
/// - `Operational`: Full deposit, trade, and withdrawal functionality.
/// - `Degraded`: Inflow (deposits/swaps) frozen; withdrawals and liquidations remain enabled
///   so user funds are never held hostage.
/// - `Halted`: Catastrophic emergency stop for administrative migration.
pub struct CircuitBreaker;

impl CircuitBreaker {
    pub fn get_state(env: &Env) -> ProtocolState {
        env.storage()
            .instance()
            .get(&PauseKey::State)
            .unwrap_or(ProtocolState::Operational)
    }

    pub fn set_state(env: &Env, new_state: ProtocolState) {
        env.storage().instance().set(&PauseKey::State, &new_state);
    }

    /// Requires the protocol to be completely operational. Rejects if Degraded or Halted.
    pub fn require_operational(env: &Env) -> Result<(), CircuitBreakerError> {
        match Self::get_state(env) {
            ProtocolState::Operational => Ok(()),
            ProtocolState::Degraded => {
                Err(CircuitBreakerError::OperationNotPermittedInDegradedMode)
            }
            ProtocolState::Halted => Err(CircuitBreakerError::ProtocolHalted),
        }
    }

    /// Allows operations permitted during degraded state (e.g. user withdrawals),
    /// but rejects if completely Halted.
    pub fn require_not_halted(env: &Env) -> Result<(), CircuitBreakerError> {
        if Self::get_state(env) == ProtocolState::Halted {
            return Err(CircuitBreakerError::ProtocolHalted);
        }
        Ok(())
    }

    /// Automated Incident Trip: If incident count exceeds threshold within `window_ledgers`,
    /// automatically trips protocol into `Degraded` mode without waiting for offline multisig signers.
    pub fn record_incident_and_trip(
        env: &Env,
        current_ledger: u32,
        max_incidents: u32,
        window_ledgers: u32,
    ) -> ProtocolState {
        let last_window: u32 = env
            .storage()
            .temporary()
            .get(&PauseKey::IncidentWindowLedger)
            .unwrap_or(0);
        let mut count: u32 = env
            .storage()
            .temporary()
            .get(&PauseKey::IncidentCount)
            .unwrap_or(0);

        if current_ledger > last_window + window_ledgers {
            count = 1;
            env.storage()
                .temporary()
                .set(&PauseKey::IncidentWindowLedger, &current_ledger);
        } else {
            count += 1;
        }

        env.storage().temporary().set(&PauseKey::IncidentCount, &count);

        if count >= max_incidents && Self::get_state(env) == ProtocolState::Operational {
            Self::set_state(env, ProtocolState::Degraded);
            return ProtocolState::Degraded;
        }

        Self::get_state(env)
    }
}
