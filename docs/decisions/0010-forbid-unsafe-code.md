# ADR-0010 — Forbid Unsafe Code

**Status:** Accepted (Provisional)

**Date:** 2026-10-05

**Recorded:** 2026-10-05 — `chore(lib): forbid unsafe code in the crate`

**Stage:** 1

**Review Trigger:** A backend that needs `unsafe`, most likely the ESP32 backend in Stage 4.

## Context

Stage 1 contains the capability traits, plain data types, devices composed from those traits, and in-memory mock backends (ADR-0002). None of this needs `unsafe`. Raw hardware access, foreign function interfaces and memory-mapped registers are possible reasons for `unsafe`, but these concerns arise primarily with real backends in Stage 2 and later.

`ind-io` is intended for building automation and, later, industrial equipment. Users in those fields reasonably want to know that the crate itself contains no `unsafe` code, and a statement in the documentation is weaker than a check the compiler makes on every build.

Whether a later backend will need `unsafe` is not known yet. Many hardware interfaces are available through safe APIs in existing crates, which keep the `unsafe` code in the dependency.

## Decision

The crate forbids `unsafe` code at the crate root:

```rust
#![forbid(unsafe_code)]
```

Unsafe blocks, unsafe functions, unsafe traits and unsafe implementations in this crate are rejected by the compiler. Unlike `deny`, `forbid` cannot be overridden by an `#[allow]` further down, so lifting it requires editing the crate attribute, which is visible in review.

The attribute covers this crate's own source. Dependencies keep their own `unsafe` code, and the attribute is not a claim about them.

The check is made by the compiler on every build, including the `thumbv6m-none-eabi` job from ADR-0002, so it needs no separate CI step.

If a later backend needs `unsafe`, the exception should be as narrow as possible. The planned response is to change the crate-level lint from `forbid` to `deny` and apply `#[allow(unsafe_code)]` only to the relevant backend module, gated by its Cargo feature, so the traits, types, devices and mocks stay free of `unsafe`. This is a plan, not a decision. It would be recorded in a new ADR that supersedes this one.

## Alternatives Considered

### No lint, rely on review

**Rejected.**

Review can miss an `unsafe` block, and nothing would stop one from appearing. Stage 1 does not need `unsafe`, so the compiler check costs nothing.

### `#![deny(unsafe_code)]`

**Not adopted in Stage 1.**

`deny` can be overridden by a more narrowly scoped `#[allow(unsafe_code)]`, allowing an exception to be introduced without changing the crate-root lint. Since Stage 1 needs no exception, the stricter lint is free. `deny` remains the likely level if a backend needs a narrow exception.

### A separate crate for backends that need `unsafe`

**Not available for backends while the traits are sealed.**

A backend in a separate crate cannot implement the sealed capability traits (ADR-0007). A separate low-level crate that exposes a safe API and is used as a dependency remains possible, and so does an existing hardware-access crate.

### Lifting `forbid` with a Cargo feature

For example, `#![cfg_attr(not(feature = "…"), forbid(unsafe_code))]`.

**Deferred.**

Cargo features are additive, so a feature enabled anywhere in the dependency graph would lift the restriction for the whole crate, including the core. Restricting the exception to one module is preferable. The choice is made with the first backend that needs `unsafe`.

## Consequences

### Positive

- The compiler rejects `unsafe` code in this crate's source.
- Reviewers do not need to audit `unsafe` in the traits, types, devices or mocks.
- The guarantee holds on every target the crate is built for, including the embedded check target.
- Introducing `unsafe` later requires an explicit, visible change to the crate attribute and a new ADR.

### Costs

- A backend that cannot be written without `unsafe` cannot be added until the lint is relaxed.
- The attribute says nothing about dependencies, so it does not make the dependency graph free of `unsafe`.
- The attribute describes this crate's source only. It is not a claim of functional safety or certification.
- Tests in the crate cannot use `unsafe` either.

## Evidence Still Missing

Stage 2 and later work must establish:

- Whether the GPIO, MQTT and ESP32 backends can be written with safe APIs from existing crates.
- Whether any backend needs `unsafe` in this crate, and whether the exception can be limited to one module behind its Cargo feature.
- Whether embedded targets need `unsafe` for shared state or interrupt handlers.
