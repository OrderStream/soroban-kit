# soroban-kit 🛡️

[![Crates.io](https://img.shields.io/crates/v/soroban-security-kit.svg)](https://crates.io/crates/soroban-security-kit)
[![CI](https://github.com/OrderStream/soroban-kit/actions/workflows/ci.yml/badge.svg)](https://github.com/OrderStream/soroban-kit/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![OrderStream Polar](https://img.shields.io/badge/Polar-OrderStream-blue.svg)](https://polar.sh/OrderStream)
[![Stellar XLM/USDC](https://img.shields.io/badge/Stellar-USDC%2FXLM-black.svg)](#direct-web3-settlement-wallets)
[![Base USDC](https://img.shields.io/badge/Base-USDC-0052FF.svg)](#direct-web3-settlement-wallets)

**Production-grade security utilities, sliding-window event deduplication, multi-tier circuit breakers, bitflag RBAC timelocks, and high-precision financial math safeguards for Stellar Soroban smart contracts.**

---

## 🌟 Modules & Capabilities

| Module | Core Primitive | Security Problem Solved |
| :--- | :--- | :--- |
| **`dedup`** | `DedupGuard` | Replay & duplicate execution suppression with persistent authorization nonces and sliding-window action dedup. |
| **`reentrancy`** | `ReentrancyGuard` | Zero-cost transient mutex protecting cross-contract calls and external token payouts. |
| **`pause`** | `CircuitBreaker` | 3-tier emergency states (`Operational`, `Degraded`, `Halted`) with auto-trip threshold monitoring. |
| **`access`** | `AccessControl` | Bitflag-packed multi-role access control (`Role::ADMIN`, `Role::MINTER`, etc.) and delayed timelock queues. |
| **`math`** | `SafeMath` | Checked 128-bit arithmetic and **Banker's Rounding (Round Half to Even)** to prevent liquidity drift. |
| **`curve`** | `CurveMath` | Piecewise kinked interest rate models for lending pools and linear decay curves for Dutch auctions. |
| **`twap`** | `TwapOracle` | Flash-loan resistant Time-Weighted Average Price accumulator and observation calculations. |
| **`storage`** | `StorageManager` | Automated persistent, instance, and temporary storage TTL extension routines preventing archival lockouts. |
| **`events`** | `EventEmitter` | Monotonically sequenced event topics ensuring zero out-of-order records in indexers. |

---

## 📦 Installation
 
Add `soroban-security-kit` directly via `cargo`:

```bash
cargo add soroban-security-kit
```

Or add it to your contract's `Cargo.toml`:

```toml
[dependencies]
soroban-security-kit = "0.1.0"
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
    CircuitBreaker::require_operational(&env).unwrap();
    // Execute deposit...
}

pub fn emergency_withdraw(env: Env, user: Address) {
    // Withdrawals remain PERMITTED in Degraded mode so user funds are never trapped
    CircuitBreaker::require_not_halted(&env).unwrap();
    // Execute withdrawal...
}
```

---

## 💳 Enterprise Pricing & Protocol Retainers

| Tier | Price | Scope | Features |
| :--- | :--- | :--- | :--- |
| **Open Source** | **$0** | Unlimited | Complete `soroban-security-kit` SDK crate |
| **Starter CI Suite** | **$499 / mo** | 10 Repositories | Automated CI invariant verification and TTL check gates |
| **Team Platform** | **$1,499 / mo** | 50 Repositories | All 10 OrderStream tools, 24/7 contract storage keeper, telemetry alarms |
| **Enterprise Protocol Retainer** | **$4,999 / mo** | Unlimited Contracts | 24/7 dedicated security engineering, custom circuit breakers, zero-decay SLA |
| **Official Soroban Audit Certificate** | **$2,500 / audit** | Single Contract | 1,000-step WASM storage TTL boundary fuzzing & cryptographic SHA-256 seal |
| **M&A IP & Security Audit** | **$15,000 one-time**| Complete Protocol | Forensic code diligence, copyleft AST scan, and executive IP certificate |

---

## ⚡ Direct Web3 Settlement Wallets

OrderStream accepts direct crypto payments with **0% intermediary platform cuts**:

* **Stellar Protocol (USDC / XLM)**:  
  `GABFQIK63R2NETJM7T673EAMZN4RJLLGP3OFUEJU5ZVTGWUKULZJNL6`  
  *(Required Memo: `400566831`)*

* **EVM Base Network (USDC / ETH)**:  
  `0x5de2b62434754bfd39ef350a3a867ed3eced5074`

* **Corporate Credit Card & Recurring Subscriptions**:  
  Subscribe online via [Polar.sh/OrderStream](https://polar.sh/OrderStream).

---

## 🚀 CLI Runner

Run the contract invariant verifier:

```bash
# Build verifier binary
cargo build --release --bin soroban_kit_verify

# Verify contract security invariants
./target/release/soroban_kit_verify --path .
```

---

## 🛡️ GitHub Action Integration

```yaml
- name: Verify Soroban Invariants
  uses: OrderStream/soroban-kit@main
  with:
    path: "."
```

---

## 🧪 Verification & Testing

```bash
cargo test
cargo build --target wasm32v1-none --release
```

---

## 📜 License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
