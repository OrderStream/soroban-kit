#![no_std]

//! # soroban-kit
//!
//! Production-ready security utilities, sliding-window event deduplication,
//! and fixed-point math safeguards for Stellar Soroban smart contracts.
//!
//! ## Modules
//! - [`dedup`]: Replay and duplicate invocation suppression with TTL renewal.
//! - [`math`]: Overflow-safe 128-bit fixed-point arithmetic with configurable rounding.
//! - [`storage`]: Efficient TTL lifecycle extensions for persistent and temporary storage.

pub mod dedup;
pub mod math;
pub mod storage;

#[cfg(test)]
mod test;
