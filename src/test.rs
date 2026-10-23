use super::access::*;
use super::curve::*;
use super::dedup::*;
use super::math::*;
use super::pause::*;
use super::reentrancy::*;
use super::twap::*;
use soroban_sdk::{
    contract, testutils::Address as _, testutils::BytesN as _, Address, BytesN, Env,
};

#[contract]
struct TestContract;

#[test]
fn test_dedup_action_recording_and_suppression() {
    let env = Env::default();
    let contract_id = env.register(TestContract, ());
    env.as_contract(&contract_id, || {
        let action_hash = BytesN::random(&env);

        assert!(!DedupGuard::is_duplicate(&env, &action_hash));
        assert!(DedupGuard::record_action(&env, &action_hash, 100));

        // Second execution must be detected as duplicate
        assert!(DedupGuard::is_duplicate(&env, &action_hash));
        assert!(!DedupGuard::record_action(&env, &action_hash, 100));
    });
}

#[test]
fn test_nonce_verification() {
    let env = Env::default();
    let contract_id = env.register(TestContract, ());
    env.as_contract(&contract_id, || {
        let user_id = 42;

        assert!(DedupGuard::verify_and_increment_nonce(&env, user_id, 1));
        assert!(DedupGuard::verify_and_increment_nonce(&env, user_id, 2));
        assert!(!DedupGuard::verify_and_increment_nonce(&env, user_id, 4));

        // Nonces must be held in persistent storage (surviving ledger TTL eviction)
        let key = DedupKey::Nonce(user_id);
        assert_eq!(
            env.storage().persistent().get::<DedupKey, u64>(&key),
            Some(2)
        );
        assert_eq!(DedupGuard::get_nonce(&env, user_id), 2);
        assert!(!env.storage().temporary().has(&key));

        // Querying an unused key returns 0
        assert_eq!(DedupGuard::get_nonce(&env, 999), 0);

        // Custom TTL horizons
        assert!(DedupGuard::verify_and_increment_nonce_with_ttl(
            &env,
            user_id,
            3,
            100,
            1000
        ));
        assert_eq!(DedupGuard::get_nonce(&env, user_id), 3);
    });
}

#[test]
fn test_storage_manager_bumps() {
    use super::storage::StorageManager;
    let env = Env::default();
    let contract_id = env.register(TestContract, ());
    env.as_contract(&contract_id, || {
        let key = 100u64;
        env.storage().persistent().set(&key, &true);
        StorageManager::bump_persistent(&env, &key, 100, 1000);

        let temp_key = 200u64;
        env.storage().temporary().set(&temp_key, &true);
        StorageManager::bump_temporary(&env, &temp_key, 100, 1000);

        StorageManager::bump_instance(&env, 100, 1000);
    });
}

#[test]
fn test_safe_math_mul_div() {
    assert_eq!(SafeMath::mul_div(1000, 50, 100).unwrap(), 500);
    assert_eq!(SafeMath::mul_div(0, 50, 100).unwrap(), 0);
    assert_eq!(
        SafeMath::mul_div(1000, 50, 0).unwrap_err(),
        MathError::DivisionByZero
    );
}

#[test]
fn test_basis_points_calculation() {
    assert_eq!(SafeMath::compute_basis_points(10_000, 250).unwrap(), 250);
    assert_eq!(
        SafeMath::compute_basis_points(5_000, 10_000).unwrap(),
        5_000
    );
}

#[test]
fn test_bankers_rounding() {
    assert_eq!(SafeMath::round_half_even(5, 2).unwrap(), 2);
    assert_eq!(SafeMath::round_half_even(7, 2).unwrap(), 4);
    assert_eq!(SafeMath::round_half_even(8, 2).unwrap(), 4);
}

#[test]
fn test_reentrancy_guard_mutex() {
    let env = Env::default();
    let contract_id = env.register(TestContract, ());
    env.as_contract(&contract_id, || {
        assert!(ReentrancyGuard::enter(&env).is_ok());
        // Second enter must fail
        assert_eq!(
            ReentrancyGuard::enter(&env).unwrap_err(),
            ReentrancyError::ReentrancyDetected
        );

        ReentrancyGuard::exit(&env);
        assert!(ReentrancyGuard::enter(&env).is_ok());
        ReentrancyGuard::exit(&env);
    });
}

#[test]
fn test_circuit_breaker_3_tier_states() {
    let env = Env::default();
    let contract_id = env.register(TestContract, ());
    env.as_contract(&contract_id, || {
        assert_eq!(CircuitBreaker::get_state(&env), ProtocolState::Operational);
        assert!(CircuitBreaker::require_operational(&env).is_ok());

        CircuitBreaker::set_state(&env, ProtocolState::Degraded);
        assert_eq!(
            CircuitBreaker::require_operational(&env).unwrap_err(),
            CircuitBreakerError::OperationNotPermittedInDegradedMode
        );
        // Degraded mode permits non-halted operations (e.g. user withdrawals)
        assert!(CircuitBreaker::require_not_halted(&env).is_ok());

        CircuitBreaker::set_state(&env, ProtocolState::Halted);
        assert_eq!(
            CircuitBreaker::require_not_halted(&env).unwrap_err(),
            CircuitBreakerError::ProtocolHalted
        );
    });
}

#[test]
fn test_rbac_bitflags() {
    let env = Env::default();
    let contract_id = env.register(TestContract, ());
    env.as_contract(&contract_id, || {
        let account = Address::generate(&env);

        assert!(!AccessControl::has_role(&env, &account, Role::ADMIN));
        AccessControl::grant_role(&env, &account, Role::ADMIN | Role::MINTER);

        assert!(AccessControl::has_role(&env, &account, Role::ADMIN));
        assert!(AccessControl::has_role(&env, &account, Role::MINTER));
        assert!(!AccessControl::has_role(&env, &account, Role::GUARDIAN));

        AccessControl::revoke_role(&env, &account, Role::ADMIN);
        assert!(!AccessControl::has_role(&env, &account, Role::ADMIN));
        assert!(AccessControl::has_role(&env, &account, Role::MINTER));
    });
}

#[test]
fn test_timelock_scheduling_and_expiry() {
    let env = Env::default();
    let contract_id = env.register(TestContract, ());
    env.as_contract(&contract_id, || {
        let action_hash = BytesN::random(&env);

        AccessControl::schedule_action(&env, &action_hash, 100, 50);

        // Before unlock ledger (100 + 50 = 150)
        assert_eq!(
            AccessControl::verify_timelock_and_consume(&env, &action_hash, 120, 20).unwrap_err(),
            AccessError::TimelockNotElapsed
        );

        // After expiration (150 + 20 = 170)
        assert_eq!(
            AccessControl::verify_timelock_and_consume(&env, &action_hash, 180, 20).unwrap_err(),
            AccessError::TimelockExpired
        );

        // Within valid execution window (150..=170)
        assert!(AccessControl::verify_timelock_and_consume(&env, &action_hash, 160, 20).is_ok());

        // Replay after execution rejected
        assert_eq!(
            AccessControl::verify_timelock_and_consume(&env, &action_hash, 165, 20).unwrap_err(),
            AccessError::ActionAlreadyExecuted
        );
    });
}

#[test]
fn test_kinked_interest_rate_curve() {
    // 80% kink (8,000 bps), 2% base rate (200 bps), 10% slope1 (1,000 bps), 50% slope2 (5,000 bps)
    let kink = 8_000;
    let base = 200;
    let slope1 = 1_000;
    let slope2 = 5_000;

    // At 40% utilization (below kink): rate = 200 + (4,000 / 8,000) * 1,000 = 700 bps (7%)
    let rate_40 = CurveMath::calculate_kinked_rate(4_000, kink, base, slope1, slope2).unwrap();
    assert_eq!(rate_40, 700);

    // At 80% utilization (at kink): rate = 200 + 1,000 = 1,200 bps (12%)
    let rate_80 = CurveMath::calculate_kinked_rate(8_000, kink, base, slope1, slope2).unwrap();
    assert_eq!(rate_80, 1_200);

    // At 90% utilization (above kink): rate = 1,200 + (1,000 / 2,000) * 5,000 = 3,700 bps (37%)
    let rate_90 = CurveMath::calculate_kinked_rate(9_000, kink, base, slope1, slope2).unwrap();
    assert_eq!(rate_90, 3_700);
}

#[test]
fn test_twap_oracle_accumulation() {
    let env = Env::default();
    let contract_id = env.register(TestContract, ());
    env.as_contract(&contract_id, || {
        // Start at t=1000 with price=100
        let _ = TwapOracle::update(&env, 100, 1000).unwrap();

        // After 100s at t=1100 with price=150: cumulative += 150 * 100 = 15,000
        let cum = TwapOracle::update(&env, 150, 1100).unwrap();
        assert_eq!(cum, 15_000);

        let twap = TwapOracle::calculate_twap(0, 15_000, 1000, 1100).unwrap();
        assert_eq!(twap, 150);
    });
}
