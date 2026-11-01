# ADR-0004 — Contract Limits and Errors

**Status:** Accepted (Provisional)

**Date:** 2026-09-28

**Recorded:** 2026-09-28 — `feat(error): add DeviceError enum`

**Amended:**

- 2026-10-04 — `refactor(traits): share MAX_PERCENT with PositionControl defaults`: brightness and position share one limit, and volume keeps its own.
- 2026-10-06 and 2026-10-07 — the mocks implement the rules: `InvalidInput` above 100 and for channel 0, and stepping that stops at the limits.
- 2026-10-22 — `refactor(error): remove unused NotInitialized variant`.
- 2026-10-23 — `feat(error): implement Display for DeviceError`.

**Stage:** 1

**Review Trigger:** A real backend that challenges the current range or error contract, or a proposal to introduce a new `DeviceError` variant.

## Context

The traits define device operations, but those operations also need deterministic behavior at their boundaries: invalid values, limits, and failed operations.

Different backends must follow the same public contract so that device behavior does not change when the backend changes.

## Decision

### Input and boundary behavior

- An explicit value outside its documented range returns `DeviceError::InvalidInput` and leaves the affected state unchanged.
- This applies to brightness, position and volume above 100, and to channel 0.
- A relative step at a defined limit is a no-op and does not return an error.
- `channel_up` saturates at the highest channel supported by the backend rather than overflowing. The mock uses `u32::MAX` as its upper limit.
- Brightness and position share one percentage limit, the crate-level `MAX_PERCENT` constant.
- Volume has its own limit because it represents a different quantity and may have different backend requirements.
- Temperature accepts any finite value. `NaN` and infinity return `DeviceError::InvalidInput`.
- No comfort-temperature range is imposed by the Stage 1 contract. The same trait may later serve HVAC applications with setpoints, such as 7 °C for chilled water, which a comfort-temperature range could reject.

### Errors

`DeviceError` is a payload-free, `Copy` enum marked `#[non_exhaustive]`. It has four variants: `HardwareFailure`, `InvalidInput`, `NotSupported` and `Timeout`.

A new error variant should be introduced with the first concrete operation or backend that produces it, together with a test covering that error path.

`HardwareFailure`, `NotSupported` and `Timeout` remain provisional because Stage 1 has no real backend to validate their semantics.

`NotInitialized` is not part of the error taxonomy. It existed in an earlier version and was removed because nothing produced it: Stage 1 has no initialization lifecycle.

`Display` messages are intended for people and logs and are not a stable machine-readable interface. Callers should match on error variants rather than parse display strings.

## Alternatives Considered

### Silent clamping of invalid input

The implementation could clamp an out-of-range value to the nearest valid value instead of returning an error.

**Rejected.** It hides caller mistakes and makes it impossible to distinguish an accepted request from a corrected one. Clamping is used only where the contract explicitly defines saturation behavior, such as `channel_up`.

### Panic on invalid input

The implementation could panic when a caller provides an invalid value.

**Rejected.** Device-control APIs should report recoverable input errors rather than terminate execution. Panics are also inappropriate for many embedded environments.

### One generic error variant

The crate could expose a single `DeviceError::Failed` or similar variant for every failure.

**Rejected.** It loses useful information for callers and makes future backend-specific error handling harder. The current taxonomy remains small while preserving meaningful distinctions.

### Large error hierarchy in Stage 1

The crate could define detailed errors for every possible hardware, protocol and transport failure before real backends exist.

**Rejected.** Those categories would be speculative. Error variants should be introduced when a concrete operation or backend provides evidence that the distinction is useful.

### Backend-specific error types

Each backend could expose its own error enum containing hardware- or protocol-specific failures.

**Not adopted in Stage 1.** The core currently uses one small, backend-independent error type. Backend-specific error reporting can be reconsidered when real backends demonstrate a need for additional diagnostic information.

### Encode all limits in dedicated types

The crate could introduce validated types such as `Percent` or a dedicated channel/volume type so invalid values cannot reach device methods.

**Not adopted in Stage 1.** The current API is simpler and the required ranges are still being validated against real backends. A validated numeric type remains a candidate for a later stage.

## Consequences

### Positive

- Invalid input produces explicit, predictable failure.
- Rejected input does not partially modify state.
- Boundary operations have deterministic behavior.
- The error taxonomy remains small and evidence-driven.
- `#[non_exhaustive]` leaves room for future error variants.

### Costs

- The current limits are not encoded in the type system; implementations must honor the documented contract.
- The contract must be documented and tested because the type system does not currently prevent every invalid value.
- Some error variants have no Stage 1 producer yet.
- `DeviceError` may need additional context when real backends are introduced.
- Making `DeviceError` carry data later would change its current API shape and remove `Copy`.
- `DeviceError` implements `Debug` and `Display` but not `core::error::Error` in Stage 1, because that trait requires Rust 1.81, above the MSRV (ADR-0001 and ADR-0002). Consequently, `DeviceError` cannot be converted directly into `Box<dyn Error>` through the standard error conversion.

## Evidence Still Missing

Stage 2 and later work must establish:

- Real backend ranges, including protocols whose levels differ from the current 0–100 contract.
- Whether GPIO, MQTT and other backend failures map cleanly to the current error variants.
- Whether backend failures require additional error context.
- Whether a validated percentage/position type should replace the current raw numeric values.
