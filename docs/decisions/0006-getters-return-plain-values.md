# ADR-0006 — Getters Return Plain Values

**Status:** Accepted (Provisional)

**Date:** 2026-09-28

**Recorded:** 2026-09-30 — the six trait commits (`feat(traits): ...`)

**Stage:** 1

**Review Trigger:** First real backend that requires refreshed or fallible reads, particularly the MQTT backend (Stage 3), or the introduction of measured-value interfaces.

## Context

Capability traits contain getter methods representing the state currently available to a device object. `is_on` is one example. Other getters return values such as speed, climate mode, position, brightness, and volume.

There is an important distinction between locally available state and freshly observed physical state.

A local mock can immediately report its state. A remote backend may instead encounter desired state, reported state, stale state, unknown state, or communication failure.

Stage 1 needs a simple synchronous API without claiming that every returned value represents the latest physical state.

## Decision

Stage 1 getters return plain values representing locally available or cached state:

```rust
fn is_on(&self) -> bool;
```

Mutating operations that may fail return:

```rust
Result<(), DeviceError>
```

The distinction is:

- Getter: reads locally available or cached state and returns a plain value.
- Mutating operation: may fail and returns `Result` where failure is meaningful.

Under the Stage 1 contract, getters read locally available state and do not perform hardware or network I/O.

The API does not guarantee that the returned value is a fresh physical-hardware reading.

Plain values keep ordinary capability access simple, and they avoid introducing `Option<T>`, `Result<T, DeviceError>`, or asynchronous return types without concrete evidence that Stage 1 requires them:

```rust
if device.is_on() {
    // ...
}
```

## Alternatives Considered

### `Result<T, DeviceError>`

**Not adopted in Stage 1.**

It would introduce error handling at every getter call, including cases where the backend already has the requested value in memory.

### `Option<T>`

**Not adopted in Stage 1.**

It could represent an unavailable or unknown value, but does not distinguish why the value is unavailable or represent communication failure.

### Dedicated state type

For example:

```rust
enum DeviceState<T> {
    Known(T),
    Unknown,
}
```

**Deferred.**

A dedicated state representation may become appropriate when real backends demonstrate the need to distinguish known, unknown, stale, or unavailable state.

### Async getters

**Not adopted in Stage 1.**

The current contract is synchronous and cache-based. Asynchronous reads should be considered when concrete backends require I/O during state retrieval. Native async methods would complicate trait-object compatibility and would require a different API design (see ADR-0009).

## Consequences

### Positive

- Simple and ergonomic getter calls.
- No unnecessary error propagation for locally available state.
- Works naturally with deterministic mocks and synchronous local backends.
- Keeps the initial API focused.

### Costs

- Plain values cannot represent unknown or unavailable state.
- Cached state may differ from the physical device state.
- Changes made outside the library may not immediately be reflected unless the backend updates its cache.
- The current getter contract does not provide a refresh mechanism.
- Getters describe configured state, not measured physical values. For example, a target temperature is not the same as the measured room temperature.
- Introducing fallible or asynchronous getters later would be a breaking API change.

## Evidence Still Missing

Stage 2 and later work must establish:

- How remote backends, particularly MQTT, learn about changes made outside the library.
- Whether cached, reported, desired, stale, and unknown state need separate representations.
- Whether a refresh or event mechanism is needed.
- Whether measured values require separate capability traits from configured values.
- Whether real hardware requires fallible or asynchronous reads.
