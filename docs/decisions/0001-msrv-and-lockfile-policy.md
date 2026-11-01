# ADR-0001 — MSRV and Lockfile Policy

**Status:** Accepted (Provisional)

**Date:** 2026-09-25

**Recorded:** 2026-09-26 — `chore: Set MSRV to Rust 1.75.0`

**Amended:**

- 2026-09-27 — `ci: add MSRV check for 1.75.0`: the MSRV is verified in CI, so the claim in `Cargo.toml` is enforced.
- 2026-09-27 — `ci: add Clippy lint check`: Clippy runs on stable only.

**Stage:** 1

**Review Trigger:** Any proposal to raise the MSRV, or the first dependency in Stage 2 or later that does not build on 1.75.0. The policy for raising the MSRV is not decided yet.

## Context

`ind-io` is a library. Its users build it with different toolchains, including embedded toolchains that lag behind the latest stable release. A library that does not state which compiler versions it supports can break a user's build with any change that uses a newer language or standard-library feature, and nothing reports the break until a user does.

Two questions have to be settled together because they interact.

The first is the minimum supported Rust version (MSRV). It must be fixed from the first release, because every later change is checked against it.

The second is the lockfile. `Cargo.lock` records exact dependency versions and carries a format version of its own. Format v4 can be read from Cargo 1.78 and is written by default from Cargo 1.83. A developer on a current toolchain therefore writes a v4 file, which Cargo 1.75.0 cannot read. Stage 1 has no dependencies, so the lockfile holds nothing of value today. The policy still has to be right before Stage 2 adds the first dependency.

## Decision

The crate declares `rust-version = "1.75.0"` in `Cargo.toml`.

The CI MSRV job installs Rust 1.75.0 and runs `cargo check` and `cargo test` on every push and pull request to `main`. A claim that is not built on that version is not treated as honest.

`Cargo.lock` is not committed. This is a library, and CI starts from a clean checkout, so the MSRV job resolves dependencies with Cargo 1.75.0 itself. When checking the MSRV locally with `cargo +1.75.0`, delete `Cargo.lock` first, because a lockfile written by a newer Cargo cannot be read.

Clippy runs on stable only. It does not show whether the crate compiles on the MSRV, and the lint set of an older Clippy differs from stable's.

Nothing in the crate may require a language or standard-library feature newer than 1.75.0. One consequence is recorded in ADR-0002: `core::error::Error` needs Rust 1.81, so `DeviceError` does not implement `Error` in Stage 1.

No policy for raising the MSRV is set. Until one is, raising it needs an `Amended` entry here that gives the reason.

## Alternatives Considered

### No declared MSRV

The crate could support only the latest stable release.

**Rejected.** Users on pinned or embedded toolchains would have no way to know which versions work, and a breaking change would stay hidden until a user's build failed.

### Declare an MSRV and verify it only locally

The version could be written in `Cargo.toml` and checked by hand.

**Rejected.** A claim that nothing enforces drifts. A dependency or a language feature can raise the real minimum without anyone noticing.

### Commit `Cargo.lock`

A committed lockfile makes every build use the same dependency versions.

**Not adopted in Stage 1.** The crate has no dependencies, so the file would hold nothing. A library's lockfile is not used by the crates that depend on it. A file written by a current toolchain also cannot be read by 1.75.0. This should be reconsidered when Stage 2 adds dependencies.

### Run Clippy on the MSRV as well

**Rejected.** It adds a second lint configuration that gives no information about compatibility.

## Consequences

### Positive

- The MSRV claim is enforced, so users on Rust 1.75.0 can rely on it.
- The lockfile format cannot break CI, because there is no committed lockfile to mismatch.
- The local check is one command and the rule is easy to state.

### Costs

- The MSRV limits the language and standard-library features the crate can use. `core::error::Error` is already excluded (ADR-0002).
- Every future dependency must build on 1.75.0. Stage 2 and later add dependencies, and one of them may force the MSRV up.
- Without a committed lockfile, CI resolves the newest compatible dependency versions. Once dependencies exist, CI can fail because a dependency released a new version, with no change in this repository.
- Deleting `Cargo.lock` before a local `+1.75.0` check is easy to forget. CI is the safety net.
- The MSRV job adds CI time on every run.

## Evidence Still Missing

Stage 2 and later work must establish:

- Whether the Stage 2 and later dependencies, such as `embedded-hal`, `rumqttc` with `tokio`, and `esp-hal`, build on Rust 1.75.0, and whether any of them forces the MSRV up.
- Whether a lockfile should be committed, or an MSRV-aware dependency resolver used, once dependencies exist.
- What policy should govern raising the MSRV: how far behind stable it may fall, and how a raise is announced and versioned.
- Whether 1.75.0 remains a useful floor for the embedded toolchains the crate targets.
