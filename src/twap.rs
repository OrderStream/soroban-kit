use crate::math::MathError;
use soroban_sdk::{contracttype, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
enum TwapKey {
    CumulativePrice,
    LastTimestamp,
}

/// Time-Weighted Average Price (TWAP) Oracle Accumulator.
///
/// Accumulates price over elapsed time seconds:
/// `price_cumulative_new = price_cumulative_old + (spot_price * delta_time)`
///
/// Shields protocols against flash-loan single-block oracle manipulation.
pub struct TwapOracle;

impl TwapOracle {
    /// Updates the cumulative price accumulator given the current spot price.
    pub fn update(env: &Env, spot_price: i128, current_timestamp: u64) -> Result<i128, MathError> {
        let mut cumulative_price: i128 = env
            .storage()
            .instance()
            .get(&TwapKey::CumulativePrice)
            .unwrap_or(0);

        if let Some(last_timestamp) = env.storage().instance().get::<_, u64>(&TwapKey::LastTimestamp) {
            let delta_time = current_timestamp.saturating_sub(last_timestamp);
            if delta_time > 0 {
                let weighted_addition = spot_price
                    .checked_mul(delta_time as i128)
                    .ok_or(MathError::Overflow)?;
                cumulative_price = cumulative_price
                    .checked_add(weighted_addition)
                    .ok_or(MathError::Overflow)?;

                env.storage()
                    .instance()
                    .set(&TwapKey::CumulativePrice, &cumulative_price);
                env.storage()
                    .instance()
                    .set(&TwapKey::LastTimestamp, &current_timestamp);
            }
        } else {
            env.storage()
                .instance()
                .set(&TwapKey::LastTimestamp, &current_timestamp);
            env.storage()
                .instance()
                .set(&TwapKey::CumulativePrice, &cumulative_price);
        }

        Ok(cumulative_price)
    }

    /// Computes the TWAP between two historical observation snapshots.
    /// `twap = (cumulative_price_2 - cumulative_price_1) / (timestamp_2 - timestamp_1)`
    pub fn calculate_twap(
        cumulative_price_start: i128,
        cumulative_price_end: i128,
        timestamp_start: u64,
        timestamp_end: u64,
    ) -> Result<i128, MathError> {
        if timestamp_end <= timestamp_start {
            return Err(MathError::DivisionByZero);
        }

        let delta_price = cumulative_price_end
            .checked_sub(cumulative_price_start)
            .ok_or(MathError::Overflow)?;
        let delta_time = (timestamp_end - timestamp_start) as i128;

        delta_price
            .checked_div(delta_time)
            .ok_or(MathError::DivisionByZero)
    }
}
