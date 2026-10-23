#![no_std]

//! # soroban-kit 🛡️
//!
//! Production-grade security utilities, sliding-window deduplication,
//! multi-tier circuit breakers, RBAC timelocks, and high-precision financial
//! math algorithms for Stellar Soroban smart contracts.
//!
//! ## Core Modules
//! - [`dedup`]: Replay and duplicate invocation suppression with TTL renewal.
//! - [`math`]: Overflow-safe 128-bit fixed-point arithmetic and Banker's rounding.
//! - [`reentrancy`]: Zero-cost re-entrancy mutex for cross-contract calls.
//! - [`pause`]: 3-tier multi-state emergency circuit breaker with auto-trip.
//! - [`access`]: Bitflag-optimized RBAC and timelocked execution queue.
//! - [`curve`]: Kinked interest rate models and linear interpolation.
//! - [`twap`]: Flash-loan resistant Time-Weighted Average Price accumulator.
//! - [`storage`]: Safe persistent, instance, and temporary TTL extension helpers.
//! - [`events`]: Monotonically sequenced event emission for reliable indexing.

pub mod access;
pub mod curve;
pub mod dedup;
pub mod events;
pub mod math;
pub mod pause;
pub mod reentrancy;
pub mod storage;
pub mod twap;

#[cfg(test)]
mod test;
