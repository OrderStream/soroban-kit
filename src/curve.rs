use crate::math::{MathError, BPS_SCALING};

/// Kinked Interest Rate Model & Financial Curves for Lending and AMMs.
pub struct CurveMath;

impl CurveMath {
    /// Computes interest rate based on utilization ratio using a kinked piecewise curve.
    ///
    /// - Below kink: `rate = base_rate + (utilization / kink) * slope1`
    /// - Above kink: `rate = base_rate + slope1 + ((utilization - kink) / (1 - kink)) * slope2`
    ///
    /// All rates and utilization are expressed in basis points (0..10,000 where 10,000 = 100%).
    pub fn calculate_kinked_rate(
        utilization_bps: i128,
        kink_bps: i128,
        base_rate_bps: i128,
        slope1_bps: i128,
        slope2_bps: i128,
    ) -> Result<i128, MathError> {
        if !(0..=BPS_SCALING).contains(&utilization_bps)
            || !(1..BPS_SCALING).contains(&kink_bps)
        {
            return Err(MathError::Overflow);
        }

        if utilization_bps <= kink_bps {
            let variable_rate = utilization_bps
                .checked_mul(slope1_bps)
                .ok_or(MathError::Overflow)?
                .checked_div(kink_bps)
                .ok_or(MathError::DivisionByZero)?;
            base_rate_bps
                .checked_add(variable_rate)
                .ok_or(MathError::Overflow)
        } else {
            let excess_utilization = utilization_bps - kink_bps;
            let excess_kink = BPS_SCALING - kink_bps;
            let excess_rate = excess_utilization
                .checked_mul(slope2_bps)
                .ok_or(MathError::Overflow)?
                .checked_div(excess_kink)
                .ok_or(MathError::DivisionByZero)?;
            base_rate_bps
                .checked_add(slope1_bps)
                .ok_or(MathError::Overflow)?
                .checked_add(excess_rate)
                .ok_or(MathError::Overflow)
        }
    }

    /// Linear interpolation between `start_value` and `end_value` across `duration_ledgers`.
    /// Useful for Dutch auctions, streaming vesting, and liquidation penalty escalation.
    pub fn linear_interpolation(
        start_value: i128,
        end_value: i128,
        elapsed_ledgers: u32,
        duration_ledgers: u32,
    ) -> Result<i128, MathError> {
        if duration_ledgers == 0 {
            return Err(MathError::DivisionByZero);
        }

        if elapsed_ledgers >= duration_ledgers {
            return Ok(end_value);
        }

        let delta = end_value
            .checked_sub(start_value)
            .ok_or(MathError::Overflow)?;
        let progress = delta
            .checked_mul(elapsed_ledgers as i128)
            .ok_or(MathError::Overflow)?
            .checked_div(duration_ledgers as i128)
            .ok_or(MathError::DivisionByZero)?;

        start_value.checked_add(progress).ok_or(MathError::Overflow)
    }
}
