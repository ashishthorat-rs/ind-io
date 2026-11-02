# ADR-0007 — Sealed Capability Traits

**Status:** Accepted (Provisional)

**Date:** 2026-09-28

**Recorded:** 2026-09-30 — `feat(traits): add capability module structure`

**Stage:** 1

**Review Trigger:** First real backend in Stage 2, or earlier if a third-party backend is requested.

## Context

ADR-0005 defines six capability traits: `PowerControl`, `SpeedControl`, `ClimateControl`, `MediaControl`, `LightControl`, and `PositionControl`.

ADR-0012 establishes that devices and backends implement the same capability traits. This allows a device to be used whenever a backend implementing the corresponding capability is expected.

These traits form part of the public API contract. Their required methods, default methods, and behavioral expectations may need refinement as real hardware and network backends are developed. If external crates can implement the traits, changes to their requirements may affect downstream implementations that the crate maintainers cannot directly review.

Sealing the traits would let the crate control which types implement them, but it would also prevent external developers from implementing the existing capability traits for their own hardware or device types. This would limit independently maintained third-party backend implementations.

## Decision

All six Stage 1 capability traits are sealed using a private supertrait.

The `Sealed` trait is declared `pub` inside a private module and re-exported only within the crate:

```rust
mod private {
    pub trait Sealed {}
}

pub(crate) use private::Sealed;

pub trait PowerControl: Sealed {
    // ...
}
```

Downstream crates cannot name `Sealed`, so they cannot implement it. Every capability trait requires it, so downstream crates cannot implement the capability traits for their own types.

External crates can still use the public capability traits as generic bounds and trait objects for the types this crate provides, subject to the trait-object compatibility requirements recorded in ADR-0009. They cannot implement these sealed traits for their own backends or devices.

New implementations of the sealed capability traits must be introduced within the crate's controlled implementation boundary and reviewed as part of the project.

## Alternatives Considered

### Fully public, unsealed capability traits

Allow downstream crates to implement the capability traits for their own backends and device types. This would support third-party implementations and an independently maintained backend ecosystem, but would leave the crate with less control over implementation behavior and make future contract changes more difficult.

**Not adopted in Stage 1.** The capability contracts have not yet been validated against real backends, and retaining control allows them to be refined before opening implementation to downstream crates. This option may be reconsidered after real backend experience or a third-party implementation request shows the need for external implementations.

### Separate device-facing and backend-facing traits

Keep device-facing traits under crate control while exposing separate traits that external crates can implement for their backends. This could allow third-party backends without opening the device-facing API, but would introduce additional traits and potentially require delegation or conversion between interfaces.

**Not adopted in Stage 1.** The need for this separation has not been demonstrated, and introducing it now would add complexity before a real backend establishes whether the two interfaces need different contracts. Reconsider when a real backend demonstrates the need for this separation.

### Feature-gated sealing

Use a Cargo feature to allow capability traits to be unsealed when external implementations are needed. This would appear to offer flexibility without permanently committing to an open implementation model, but Cargo features are additive across the dependency graph, making the sealing policy dependent on the combined feature configuration rather than a predictable crate-level boundary.

**Rejected.** The implementation policy should not change unpredictably depending on which features other dependencies enable.

## Consequences

### Positive

- The crate retains control over which types implement the capability traits.
- Required methods and default-method behavior can be reviewed alongside in-crate implementations.
- Capability contracts can be refined before downstream implementation compatibility becomes a concern.
- The implementation policy is explicit and consistent across all six Stage 1 capability traits.
- Unsealing later is not a breaking change, whereas sealing later would break every downstream implementation.

### Costs

- External crates cannot implement the sealed capability traits for their own hardware backends or device types.
- Types defined outside the crate cannot implement these sealed traits, so they cannot directly satisfy generic bounds or serve as the concrete implementation behind these trait objects. They can still compose or wrap `ind-io` devices, but composition does not make the external type an implementation of a sealed trait.
- While the traits are sealed, every backend must live inside this crate, behind Cargo features. Separate backend crates, and an independently maintained third-party backend ecosystem, are not possible unless the traits are unsealed.
- In-crate backend implementations require maintainer review.
- Failure-path tests for implementations need crate-internal test doubles because external integration tests cannot implement the sealed traits.
- Future documentation must explain the sealing restriction and direct prospective backend contributors to the project.

## Evidence Still Missing

Stage 2 and later work must establish:

- Whether real local and remote backends can be supported without frequent changes to the public capability API.
- Whether external developers need to implement capability traits independently.
- Whether separate, open backend-facing traits would provide a useful extension point without weakening the device API.
- Whether crate-internal test doubles adequately support failure-path and integration testing.
- Whether keeping every backend in this crate, behind Cargo features, remains manageable as protocols and dependencies grow.
- Whether sealing all six traits remains appropriate as the project expands into additional protocols and industrial use cases.
- Whether the comparable `embedded-hal` model, in which separate platform crates implement core traits, provides a meaningful advantage for backend adoption and whether that model is appropriate for `ind-io`.
