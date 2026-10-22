use soroban_sdk::{contracttype, symbol_short, Env, Symbol, Val, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
enum EventKey {
    SequenceCounter,
}

/// Standardized event emitter with deterministic global sequence ordering.
///
/// Attaches an autoincrementing monotonic sequence integer to contract events,
/// preventing out-of-order event ingestion in external indexers (Mercury, Zephyr, StellarLens).
pub struct EventEmitter;

impl EventEmitter {
    /// Emits a structured event with an incremented sequence index.
    pub fn emit_sequenced(env: &Env, topic: Symbol, data: Val) -> u64 {
        let seq: u64 = env
            .storage()
            .instance()
            .get(&EventKey::SequenceCounter)
            .unwrap_or(0)
            + 1;
        env.storage()
            .instance()
            .set(&EventKey::SequenceCounter, &seq);

        let mut topics = Vec::new(env);
        topics.push_back(topic);
        topics.push_back(symbol_short!("seq"));

        env.events().publish(topics, (seq, data));
        seq
    }
}
