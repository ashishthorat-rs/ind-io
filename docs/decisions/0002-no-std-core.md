# ADR-0002 — `no_std` Core

**Status:** Accepted

**Date:** 2026-09-25

**Recorded:** 2026-09-27 — `feat: add lib.rs with no_std core and public module structure`. The same day, `ci: add no_std target check` began enforcing it.

**Stage:** 1

**Review Trigger:** The first backend that needs `alloc` or `std` (Stage 2 or later), when the feature layout is decided.

## Context

`ind-io` abstracts device control for use on very different targets. The planned backends range from GPIO and ESP32 firmware, which have no operating system and may have no heap, to MQTT clients on a Raspberry Pi, which have `std`.

A crate that depends on `std` cannot run on the first group. A crate that is `no_std` can run on both, because `core` is available everywhere. Whether a crate can be `no_std` is also hard to change later: once public types or signatures use `std`-only items, removing them breaks users.

Stage 1 contains only traits, plain data types, devices composed from the traits, and in-memory mock backends. None of that needs an operating system, a heap or I/O.

## Decision

The `ind-io` core is `#![no_std]`. The library, including the traits, the types, the devices and the mock backends, uses only `core`. It uses neither `alloc` nor `std`.

CI enforces this by compiling the library for `thumbv6m-none-eabi`, a bare-metal ARM Cortex-M0 target that has no operating system and no `std`. Any use of `std` in the library fails that job. The `#![no_std]` declaration alone does not give this guarantee, because it does not stop a dependency from using `std` or an explicit `extern crate std;`, and the normal host build accepts both.

The decision concerns the core crate. When a later backend needs `alloc` or `std`, that support is added through Cargo features that only add items, and the core stays `no_std`. Cargo features are additive across the dependency graph, so a feature must not invalidate or change the guarantees of the `no_std` core. The feature layout is decided when the first such backend is written, and not by this ADR.

## Alternatives Considered

### A `std` crate

The crate could use `std` throughout.

**Rejected.** This would rule out the embedded backends the crate exists to support, and the choice cannot be undone without breaking users once `std` types appear in the public API.

### `no_std` with a required `alloc`

The core could depend on `alloc` for collections and boxed types.

**Rejected.** Stage 1 needs no heap, and requiring `alloc` would exclude targets without a heap allocator for no benefit. A backend that needs `alloc` can still enable it through a feature.

### `std` as a default feature that can be turned off

The crate could use `std` by default and become `no_std` when the default features are disabled.

**Rejected.** Features are additive, so any one crate in a user's dependency graph that enables `std` turns it on for everyone. For a crate whose guarantee is `no_std`, that is too fragile. The core is `no_std` unconditionally, and `std` can only be added.

## Consequences

### Positive

- The core runs on embedded targets and on hosted targets without change.
- The guarantee is checked on every push and pull request, so it cannot erode without a failing build.
- The public API contains no `std`-only types, so adding backends later does not break it.

### Costs

- `core::error::Error` needs Rust 1.81, which is above the MSRV of 1.75.0 (ADR-0001). `DeviceError` therefore implements `Debug` and `Display` (ADR-0004) but not `Error` in Stage 1. Users cannot convert it with `?` into `Box<dyn Error>`. `std::error::Error` can be implemented behind a `std` feature in Stage 2 without raising the MSRV.
- Only the library is compiled for the embedded target. The unit and integration tests run on the host with the standard test harness, so they do not show how the code behaves on an embedded device.
- Features that enable `alloc` or `std` must be designed with care, because they are additive and apply to the whole dependency graph.

## Evidence Still Missing

Stage 2 and later work must establish:

- Whether real backends for GPIO, MQTT and ESP32 can be written with the core and a small set of additive features.
- Whether a `std` feature can add `std::error::Error` and the other `std` conveniences without affecting the core's behavior.
- Whether the `thumbv6m-none-eabi` check should be extended to additional targets, for example one with atomic operations.
- Whether the mock backends should stay available by default once real backends exist, or move behind a feature (ADR-0003).
