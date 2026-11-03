# ADR-0009 — Default Methods Stay Dyn Compatible

**Status:** Accepted

**Date:** 2026-10-04

**Recorded:** 2026-10-04 — `fix(traits): remove unnecessary Self: Sized from PowerControl defaults`

**Amended:**

- 2026-10-07 — `feat(mock): add MockPower backend`: guard test `default_methods_work_through_a_trait_object`.
- 2026-10-12 to 2026-10-22 — the device tests call the defaults through trait objects.

**Stage:** 1

**Review Trigger:** First backend with a native `toggle` or `reset` (Stage 2), or a new method that cannot be dyn compatible.

## Context

Capability traits may contain default methods that can be implemented generically using primitive operations. `PowerControl` has two:
```rust
fn toggle(&mut self) -> Result<(), DeviceError> {
    if self.is_on() {
        self.turn_off()
    } else {
        self.turn_on()
    }
}

fn reset(&mut self) -> Result<(), DeviceError> {
    self.turn_off()?;
    self.turn_on()
}
```

These methods do not require every device to implement them independently. ADR-0008 records the behavior `reset` must have.

`toggle` and `reset` first merged on 2026-09-30 in `feat(traits): add PowerControl capability` with a `where Self: Sized` clause, on the belief that default methods calling other trait methods require it. They do not. The clause makes the methods unavailable through a trait object such as `&mut dyn PowerControl`. The commit named in `Recorded` removed it.

## Decision

Capability trait default methods that are intended to be generally usable through trait objects must not carry unnecessary `Self: Sized` constraints.

The Stage 1 `PowerControl` contract provides these as default methods without `where Self: Sized`:
```rust
fn toggle(&mut self) -> Result<(), DeviceError>
fn reset(&mut self) -> Result<(), DeviceError>
```

The capability traits remain dyn compatible (the current Rust terminology for traits that can be used as trait objects), and these default methods are callable through trait-object references. Application code can then operate on devices of different types through a common capability, without knowing the concrete type.

Not every method must be dyn compatible. A method that genuinely cannot satisfy the dyn-compatibility requirements, for example because it is generic or returns `Self`, may be excluded with `where Self: Sized`. The trait stays dyn compatible, the method is unavailable through trait objects, and adding one must be a deliberate API decision.

A default method is defined only when its behavior is valid for every implementation of the capability. `toggle` qualifies for `PowerControl` because the capability defines `is_on`, `turn_on` and `turn_off`. A method is not added as a default merely because it is convenient.

Tests guard the Stage 1 contract by calling the default methods through `&mut dyn PowerControl` references in the mock and device tests.

## Alternatives Considered

### Add `Self: Sized` to all default methods

**Rejected.**

This unnecessarily prevents otherwise valid default methods from being called through trait objects.

### Require every device to implement `toggle` and `reset`

**Rejected.**

This duplicates generic behavior and creates unnecessary implementation burden and opportunities for inconsistent semantics.

### Implement the defaults as inherent device methods

**Rejected.**

That would make the behavior device-specific rather than part of the capability contract and would prevent generic code from using the operation through `dyn PowerControl`.

## Consequences

### Positive

- Default behavior is defined in one place.
- Capability semantics remain consistent.
- Trait-object users can call the default methods.
- Device implementations remain smaller.
- Static and dynamic dispatch use the same semantic contract.
- Generic control layers can operate on heterogeneous devices without requiring device-specific implementations of the defaults.

### Costs

- Dyn compatibility must be considered when adding future trait methods.
- Some future methods may legitimately require `Self: Sized` and therefore be unavailable through trait objects.
- Default methods are appropriate only when their semantics are valid for every implementation of the capability.
- A backend-specific native implementation of an operation cannot be assumed merely because the capability provides a default method.

## Evidence Still Missing

Stage 2 and later work must establish:

- Whether backend-native implementations of these operations provide a meaningful benefit over the capability defaults.
- Whether forwarding default operations to backend-native commands can be done without compromising dyn compatibility.
- Whether real hardware exposes operations whose semantics differ materially from the Stage 1 generic implementations.
