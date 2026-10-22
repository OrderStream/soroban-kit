# soroban-kit 🛡️

[![CI](https://github.com/OrderStream/soroban-kit/actions/workflows/ci.yml/badge.svg)](https://github.com/OrderStream/soroban-kit/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Soroban SDK](https://img.shields.io/badge/Soroban%20SDK-v22.0.0-purple.svg)](https://stellar.org/soroban)

Production-ready security utilities, sliding-window event deduplication, and fixed-point math safeguards for Stellar Soroban smart contracts.

---

## 🚀 Features

- **Sliding-Window Deduplication (`DedupGuard`)**:
  - Replay and duplicate action suppression using Soroban temporary storage.
  - Zero permanent storage footprint with automatic ledger TTL renewal.
  - Monotonically increasing sequential nonce verification for off-chain signed permits.

- **Defensive Fixed-Point Math (`SafeMath`)**:
  - Overflow-checked `mul_div` intermediate precision scaling.
  - Basis point calculations (`compute_basis_points`) with strict upper bound validation.
  - **Banker's Rounding (Round Half to Even)**: Eliminates statistical upward balance drift across high-frequency payment channels.

- **Storage Lifecycle Helper (`StorageManager`)**:
  - Safe TTL renewal thresholds for instance and temporary contract data.

---

## 📦 Installation

Add `soroban-kit` to your `Cargo.toml`:

```toml
[dependencies]
soroban-kit = { git = "https://github.com/OrderStream/soroban-kit.git" }
```

---

## 💡 Code Examples

### 1. Sliding-Window Deduplication & Replay Guard

```rust
use soroban_kit::dedup::DedupGuard;
use soroban_sdk::{BytesN, Env};

pub fn execute_payout(env: Env, tx_hash: BytesN<32>, recipient: Address, amount: i128) {
    // Window of 17,280 ledgers (~24 hours on Stellar)
    let is_new = DedupGuard::record_action(&env, &tx_hash, 17_280);
    if !is_new {
        panic!("Duplicate transaction hash rejected");
    }

    // Proceed with payout...
}
```

### 2. High-Precision Fee Calculation & Banker's Rounding

```rust
use soroban_kit::math::SafeMath;

// Calculate 1.5% protocol fee (150 basis points)
let fee = SafeMath::compute_basis_points(10_000, 150).unwrap();
assert_eq!(fee, 150);

// Round half to even (banker's rounding)
let rounded = SafeMath::round_half_even(7, 2).unwrap();
assert_eq!(rounded, 4); // 3.5 rounds to even 4
```

---

## 🧪 Verification & Testing

Run unit tests via Cargo:

```bash
cargo test
```

Build WebAssembly release binary:

```bash
cargo build --target wasm32v1-none --release
```

---

## 📜 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
