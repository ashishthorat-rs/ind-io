# ADR-0003 — Public API Surface

**Status:** Accepted (Provisional)

**Date:** 2026-09-26

**Recorded:** 2026-09-27 — `feat: add lib.rs with no_std core and public module structure`

**Amended:**

- 2026-09-27 — `fix(lib): remove premature pub use re-exports`: root re-exports are added only when the item exists.
- 2026-10-04 — `feat(lib): re-export DeviceError and type enums at crate root`. `MAX_PERCENT` stays `pub(crate)` in `refactor(traits): share MAX_PERCENT with PositionControl defaults`.
- 2026-10-07 — mocks stay namespaced: `feat(mock): export all mock backends from mock module`.
- 2026-10-12 to 2026-10-22 — the devices are exported at the crate root one by one, AirConditioner first and Thermostat last.

**Stage:** 1

**Review Trigger:** The 0.5.0 API freeze, or the first example that needs a different import path.

## Context

Everything a crate exports becomes part of its public API, and changes to those public items can be subject to semantic-versioning compatibility rules. Moving or renaming an item later is a breaking change. The crate therefore has to decide, early, which items users can reach and by which paths.

`ind-io` has five groups of public items: the capability traits, the devices, the error type, the shared types such as `Speed`, and the mock backends. The traits and the mocks differ from the others in how they are used. The traits are the contract, and they must be in scope for their methods to be called. The mocks are test doubles, not production backends.

The contracts themselves are decided elsewhere: the capability traits in ADR-0005, sealing in ADR-0007, the devices in ADR-0012. This ADR decides only what is exported and where.

## Decision

The crate has five public modules: `devices`, `error`, `mock`, `traits` and `types`.

The crate root re-exports the items users need most often:

- the nine devices: `Switch`, `Fan`, `Light`, `Television`, `AirConditioner`, `AirPurifier`, `Heater`, `Thermostat` and `Curtain`;
- `DeviceError`;
- the enums `ClimateMode`, `Speed` and `SwingMode`.

The traits and the mock backends are not re-exported at the root. They stay under their modules, `ind_io::traits` and `ind_io::mock`. Users import traits explicitly:

```rust
use ind_io::mock::MockPower;
use ind_io::traits::PowerControl;
use ind_io::Switch;
```

The trait must be in scope for its trait methods to be called with method syntax, so a root re-export would not remove the need to import the trait. It would only add a second path before the paths are proven.

A prelude is deferred until the 0.5.0 API freeze. Users import traits explicitly until then.

A root `pub use` is added only after the item it exports exists. In practice each device is added in one commit and exported in the next. The first version of `lib.rs` re-exported items that did not exist yet, and that was reverted.

Items that exist only to support the implementation stay `pub(crate)`. This applies to the `Sealed` supertrait (ADR-0007) and to `MAX_PERCENT` (ADR-0004).

## Alternatives Considered

### Modules only

Users could import everything through its module, with no root re-exports.

**Not adopted.** The devices are the entry point to the crate, and `ind_io::devices::Fan` is longer than `ind_io::Fan` for no gain. Users look at the crate root first.

### Flat re-export of everything

The root could re-export the traits and the mocks as well.

**Not adopted.** It fixes a second path for every trait, and it puts test doubles beside the production devices. Keeping the mocks under `mock` marks them as a different kind of item.

### A prelude now

A `prelude` module could re-export the traits and the common types.

**Deferred.** A prelude is a promise about which names a glob import brings in, and adding to it later can cause name clashes in users' code. The set of items is not yet proven, so it waits for the API freeze.

### Gate the mocks behind a feature

The `mock` module could be compiled only when a `mock` feature is enabled.

**Not decided here.** The mocks are the only backends in Stage 1, and the examples and tests need them. The question returns when real backends exist.

## Consequences

### Positive

- The common items are one import away: `ind_io::Fan`, `ind_io::DeviceError`.
- The traits and the mocks keep clear names that show what they are.
- The paths in the public API stay few until they are proven by examples and real backends.
- Each export is a deliberate, reviewable change, one device per commit.

### Costs

- Devices, `DeviceError` and the enums are reachable by two paths, for example `ind_io::Fan` and `ind_io::devices::Fan`. Both paths are public API and therefore become compatibility commitments.
- Users must import each trait they use. Examples and documentation need the extra `use` lines until a prelude exists.
- The mocks are part of the default crate build and public API in Stage 1. Changing them is a semantic-versioning concern, even though they are test doubles.
- The decision concerns the Stage 1 layout. The module layout may need to change when real backends are added.

## Evidence Still Missing

Stage 2 and later work must establish:

- Whether the Phase 8 examples and real backends make the trait imports awkward enough to justify a prelude, and what it should contain.
- Whether the mock backends should move behind a feature once real backends exist.
- Where real backends should live in the module layout.
- Whether the root re-exports should change at the 0.5.0 API freeze.
