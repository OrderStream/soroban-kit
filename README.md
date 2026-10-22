# soroban-kit 🛡️

[![CI](https://github.com/OrderStream/soroban-kit/actions/workflows/ci.yml/badge.svg)](https://github.com/OrderStream/soroban-kit/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Soroban SDK](https://img.shields.io/badge/Soroban%20SDK-v22.0.0-purple.svg)](https://stellar.org/soroban)
[![WASM Ready](https://img.shields.io/badge/WASM-wasm32v1--none-brightgreen.svg)](https://github.com/OrderStream/soroban-kit)

Production-grade security utilities, sliding-window event deduplication, multi-tier circuit breakers, bitflag RBAC timelocks, and high-precision financial math safeguards for Stellar Soroban smart contracts.

---

## 🌟 Modules & Capabilities

| Module | Core Primitive | Security Problem Solved |
| :--- | :--- | :--- |
| **`dedup`** | `DedupGuard` | Replay & duplicate execution suppression using temporary storage with zero permanent state pollution. |
| **`reentrancy`** | `ReentrancyGuard` | Zero-cost transient mutex protecting cross-contract calls and external token payouts. |
| **`pause`** | `CircuitBreaker` | 3-tier emergency states (`Operational`, `Degraded`, `Halted`) with auto-trip threshold monitoring. |
| **`access`** | `AccessControl` | Bitflag-packed multi-role access control (`Role::ADMIN`, `Role::MINTER`, etc.) and delayed timelock queues. |
| **`math`** | `SafeMath` | Checked 128-bit arithmetic and **Banker's Rounding (Round Half to Even)** to prevent liquidity drift. |
| **`curve`** | `CurveMath` | Piecewise kinked interest rate models for lending pools and linear decay curves for Dutch auctions. |
| **`twap`** | `TwapOracle` | Flash-loan resistant Time-Weighted Average Price accumulator and observation calculations. |
| **`storage`** | `StorageManager` | Automated instance and temporary storage TTL extension routines preventing archival lockouts. |
| **`events`** | `EventEmitter` | Monotonically sequenced event topics ensuring zero out-of-order records in indexers. |

---

## 📦 Installation

Add `soroban-kit` to your contract's `Cargo.toml`:

```toml
[dependencies]
soroban-kit = { git = "https://github.com/OrderStream/soroban-kit.git" }
```

---

## 💡 Code Walkthroughs

### 1. Re-entrancy Protection for Cross-Contract Payouts

```rust
use soroban_kit::reentrancy::ReentrancyGuard;
use soroban_sdk::{Address, Env};

pub fn withdraw_collateral(env: Env, user: Address, amount: i128) {
    user.require_auth();

    // Guard against malicious cross-contract re-entrancy
    ReentrancyGuard::with_guard(&env, || {
        // Safe token transfer logic...
    }).unwrap();
}
```

### 2. Multi-Tier Circuit Breaker (Never Trap User Withdrawals)

```rust
use soroban_kit::pause::{CircuitBreaker, ProtocolState};
use soroban_sdk::Env;

pub fn deposit(env: Env, amount: i128) {
    // Deposits are strictly blocked if protocol is Degraded or Halted
    CircuitBreaker::require_operational(&env).unwrap();
    // Execute deposit...
}

pub fn emergency_withdraw(env: Env, user: Address) {
    // Withdrawals remain PERMITTED in Degraded mode so user funds are never trapped
    CircuitBreaker::require_not_halted(&env).unwrap();
    // Execute withdrawal...
}
```

### 3. Bitflag Role-Based Access Control & Timelock

```rust
use soroban_kit::access::{AccessControl, Role};
use soroban_sdk::{Address, Env};

pub fn mint_reward(env: Env, caller: Address, recipient: Address, amount: i128) {
    // Requires MINTER role in a single bitwise comparison
    AccessControl::require_role(&env, &caller, Role::MINTER).unwrap();
    // Mint token...
}
```

### 4. Kinked Interest Rate Model for Lending Protocols

```rust
use soroban_kit::curve::CurveMath;

// 80% Kink (8,000 bps), 2% Base (200 bps), 10% Slope1 (1,000 bps), 50% Slope2 (5,000 bps)
let rate = CurveMath::calculate_kinked_rate(
    utilization_bps, // e.g. 8,500 bps (85%)
    8_000,
    200,
    1_000,
    5_000,
).unwrap();
```

---

## 🧪 Verification & Testing

All modules are verified with automated unit tests:

```bash
cargo test
```

Compiled for WebAssembly release target:

```bash
cargo build --target wasm32v1-none --release
```

---

## 📜 License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
