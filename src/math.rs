use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MathError {
    Overflow = 1,
    DivisionByZero = 2,
    PrecisionLoss = 3,
}

pub const BPS_SCALING: i128 = 10_000;
pub const PRECISION_18: i128 = 1_000_000_000_000_000_000;

/// High-precision fixed-point math safeguards for financial protocols.
pub struct SafeMath;

impl SafeMath {
    /// Safe multiplication followed by division with ceiling/floor control.
    /// Calculates `(amount * numerator) / denominator` using intermediate 128-bit checks.
    pub fn mul_div(amount: i128, numerator: i128, denominator: i128) -> Result<i128, MathError> {
        if denominator == 0 {
            return Err(MathError::DivisionByZero);
        }
        if amount == 0 || numerator == 0 {
            return Ok(0);
        }

        // Detect signed overflow
        let prod = amount.checked_mul(numerator).ok_or(MathError::Overflow)?;
        let result = prod.checked_div(denominator).ok_or(MathError::Overflow)?;
        Ok(result)
    }

    /// Computes percentage fee in basis points (1 bps = 0.01%).
    /// `amount * bps / 10,000`
    pub fn compute_basis_points(amount: i128, bps: i128) -> Result<i128, MathError> {
        if !(0..=BPS_SCALING).contains(&bps) {
            return Err(MathError::Overflow);
        }
        Self::mul_div(amount, bps, BPS_SCALING)
    }

    /// Banker's Rounding (Round Half to Even)
    /// Minimizes systematic upward bias across repeated micro-transactions.
    pub fn round_half_even(numerator: i128, denominator: i128) -> Result<i128, MathError> {
        if denominator == 0 {
            return Err(MathError::DivisionByZero);
        }

        let quotient = numerator / denominator;
        let remainder = (numerator % denominator).abs();
        let half = denominator.abs() / 2;
        let is_exact_half = (remainder * 2) == denominator.abs();

        if remainder > half || (is_exact_half && quotient % 2 != 0) {
            if numerator >= 0 {
                Ok(quotient + 1)
            } else {
                Ok(quotient - 1)
            }
        } else {
            Ok(quotient)
        }
    }
}
