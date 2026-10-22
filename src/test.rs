use super::dedup::*;
use super::math::*;
use soroban_sdk::{testutils::BytesN as _, BytesN, Env};

#[test]
fn test_dedup_action_recording_and_suppression() {
    let env = Env::default();
    let action_hash = BytesN::random(&env);

    assert_eq!(DedupGuard::is_duplicate(&env, &action_hash), false);
    assert_eq!(DedupGuard::record_action(&env, &action_hash, 100), true);

    // Second execution must be detected as duplicate
    assert_eq!(DedupGuard::is_duplicate(&env, &action_hash), true);
    assert_eq!(DedupGuard::record_action(&env, &action_hash, 100), false);
}

#[test]
fn test_nonce_verification() {
    let env = Env::default();
    let user_id = 42;

    assert_eq!(
        DedupGuard::verify_and_increment_nonce(&env, user_id, 1),
        true
    );
    assert_eq!(
        DedupGuard::verify_and_increment_nonce(&env, user_id, 2),
        true
    );
    // Out of order nonce should fail
    assert_eq!(
        DedupGuard::verify_and_increment_nonce(&env, user_id, 4),
        false
    );
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
    // 250 bps = 2.50% of 10,000 = 250
    assert_eq!(SafeMath::compute_basis_points(10_000, 250).unwrap(), 250);
    // 100% = 10,000 bps
    assert_eq!(
        SafeMath::compute_basis_points(5_000, 10_000).unwrap(),
        5_000
    );
}

#[test]
fn test_bankers_rounding() {
    // Round half to even:
    // 5 / 2 = 2.5 -> rounds to even integer 2
    assert_eq!(SafeMath::round_half_even(5, 2).unwrap(), 2);
    // 7 / 2 = 3.5 -> rounds to even integer 4
    assert_eq!(SafeMath::round_half_even(7, 2).unwrap(), 4);
    // 8 / 2 = 4.0 -> 4
    assert_eq!(SafeMath::round_half_even(8, 2).unwrap(), 4);
}
