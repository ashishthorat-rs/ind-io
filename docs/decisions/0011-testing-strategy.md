# ADR-0011 — Testing Strategy

**Status:** Accepted

**Date:** 2026-10-05

**Recorded:** 2026-10-06 — the first tests, in the mock backend commits (`feat(mock): add MockClimate backend`, `feat(mock): add MockLight backend`, `feat(mock): add MockMedia backend`)

**Amended:**

- 2026-10-12 — `test(air_conditioner): add AirConditioner integration tests`: integration tests begin, one file per device.
- 2026-10-13 — `test(air_purifier): add AirPurifier integration tests`: each test name states one behavioral claim.
- 2026-10-23 — `test(error): add DeviceError Display tests`: `tests/error_tests.rs` follows the same layout.

**Stage:** 1

**Review Trigger:** Failure-path tests with crate-internal test doubles (Stage 2), or a change to the sealing policy.

## Context

Stage 1 defines behavior by contract: limits and errors (ADR-0004), plain-value getters (ADR-0006), independent capabilities (ADR-0008) and default methods that work through trait objects (ADR-0009). There is no hardware to test against, so the contract has to be verified against the in-memory mock backends.

The library remains `no_std`. Tests are compiled and run on the host using Rust's standard test harness, so test code can use `std` without changing the library's `no_std` contract.

## Decision

Stage 1 tests have two levels.

**Mock unit tests** sit in the source file of each mock backend, in a `#[cfg(test)] mod tests`. They verify the backend's own contract: its initial state, its limits, and its error cases.

**Integration tests** sit in `tests/`. Each source file that has integration tests gets one file, named `<source-file>_tests.rs`. For example, `tests/air_conditioner_tests.rs` tests `devices/air_conditioner.rs`, and `tests/error_tests.rs` tests `error.rs`. They use only the public API (`ind_io::…`) and build each device from mock backends. Devices have no unit tests in their source files.

Device integration tests verify the contract at the device level:

- Each capability reads and writes through to its backend.
- The independence rules hold (ADR-0008), including `reset` leaving the device on with its settings unchanged.
- The device is usable through trait objects (ADR-0009).

Each test name states one behavioral claim, for example `reset_leaves_the_device_on_with_settings_unchanged`. A failing test then names the promise that broke.

CI runs the tests as one of five checks on every pull request: formatting, Clippy on stable, the tests, the tests on the minimum supported Rust version, 1.75.0 (ADR-0001), and a `no_std` compile check of the library for `thumbv6m-none-eabi` (ADR-0002). The embedded job covers the library only, and the tests run on the host.

The Stage 1 mocks do not simulate backend failures. Tests therefore cover the errors the contract defines for them, which are `InvalidInput` for out-of-range values with state left unchanged (ADR-0004). Injecting `HardwareFailure`, `NotSupported` and `Timeout` is deferred until a backend produces them.

## Alternatives Considered

### Unit tests only

**Not adopted.**

Tests inside the crate can use private items and would not notice a missing re-export. Integration tests compile against the public API, so they also check what a user can actually import (ADR-0003).

### A shared integration test file

All integration tests could live in one file.

**Not adopted.**

One file per source file keeps the location of a test predictable and the files small.

### Failure-injection test doubles in Stage 1

**Deferred.**

A failure-injection test double must implement the sealed capability traits, so it must be defined inside the crate, for example in crate-internal test support. No Stage 1 backend produces these errors yet (ADR-0004), so the doubles would test behavior nothing exhibits. Reconsider with the first backend that produces them.

### Property-based or fuzz testing

**Deferred.**

The Stage 1 domain is small: limits at 0 and 100, a few enums and finite floats. Boundary and representative values are tested by example. Property-based testing is deferred until a larger input space or more complex invariants justify the additional tooling and dev-dependencies, which must also remain compatible with the MSRV (ADR-0001).

### Hardware-in-the-loop tests

**Not applicable in Stage 1.**

There is no hardware backend. Reconsider with the first real backend in Stage 2.

## Consequences

### Positive

- The Stage 1 contract is verified without hardware.
- Test names read as a specification of the behavior.
- Integration tests exercise the public API as a user sees it.
- The mapping from a source file to its tests is predictable.

### Costs

- Tests run on the host, not on the embedded target, so they do not show how the code behaves on a device (ADR-0002).
- The mocks are the only backend, so the tests show that the contract is consistent, not that real hardware can satisfy it.
- No failure path beyond invalid input is tested, because nothing produces it yet.
- The naming and layout conventions are not mechanically enforced.
- Some behavior is checked at both levels, in a mock's unit tests and in the device tests.

## Evidence Still Missing

Stage 2 and later work must establish:

- How real backends are tested without hardware in CI, and whether hardware-in-the-loop tests are needed.
- Whether crate-internal test doubles adequately cover failure paths, and where they should live.
- Whether the mock contract tests can become a shared conformance suite that every backend must pass.
- Whether property-based tests would find problems that example tests miss.
- Whether some tests should run on an embedded target.
