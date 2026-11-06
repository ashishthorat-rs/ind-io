# ADR-0012 — Devices Implement Capability Traits

**Status:** Accepted (Provisional)

**Date:** 2026-10-10

**Recorded:** 2026-10-12 — `feat(devices): add AirConditioner device` and `feat(devices): add device module structure` the same day

**Stage:** 1

**Review Trigger:** First real backend (Stage 2), the first multi-device grouping, or a device requiring behavior beyond composition and delegation.

## Context

`ind-io` separates recognizable device abstractions from the mechanisms used to control them.

A heater, for example, combines capabilities such as power and climate control. The underlying implementations may eventually use GPIO, serial communication, MQTT, vendor SDKs, network protocols, or other hardware interfaces.

This crate needs a clear relationship between device types, public capability contracts, and the implementations that perform the actual operations.

Two architectural questions must be distinguished:

- How should devices expose their capabilities to application code?
- Should the public capability traits also serve as the interfaces implemented by underlying backends, or should separate backend-facing interfaces be introduced?

Stage 1 needs a design that supports generic control, mock-backed testing, and device composition without introducing abstraction layers before concrete hardware requirements justify them.

## Decision

For Stage 1, device types implement the public capability traits corresponding to the capabilities they compose.

Backends implement the same capability traits as devices. A device can therefore be used anywhere a value implementing the corresponding capability trait is accepted.

Device structs compose the required capability implementations and delegate primitive operations to them. The device acts as the composition boundary, while the capability traits define the public behavioral contract.

Devices add no behavior of their own. They do not duplicate capability semantics or introduce application policy. Backend errors are propagated unchanged, and getters obtain the state exposed by their composed implementations.

Trait bounds must be scoped to the requirements of each implementation. A device's implementation of one capability requires only the component needed for that capability. Constructors, however, require all components necessary to construct a valid device.

Default methods belong to the public capability traits and operate through the device's own trait implementation. They are not automatically forwarded to a backend's corresponding default-method override; see ADR-0009 for why default methods stay callable through trait objects.

This decision deliberately avoids introducing separate backend-facing traits before real backends have shown the need.

## Alternatives Considered

### Inherent device methods only

Each device could expose its operations exclusively through methods defined directly on its own type.

**Rejected as the primary API.** This would make generic control code depend on concrete device types and weaken capability-based composition. Shared behavior would also be harder to express consistently across devices.

### Both inherent methods and capability traits

Each device could expose both inherent methods and trait methods for the same operations.

**Rejected.** Maintaining two ways to access the same functionality increases the public API surface, duplicates method definitions across device types, and risks inconsistent behavior over time.

### Separate device-facing and backend-facing traits

The public capability traits could describe application-facing semantics, while separate backend interfaces define the mechanisms required to perform those operations.

**Not adopted in Stage 1.** A second interface layer would introduce additional abstractions and implementation work before real backends have shown the need. This separation remains a possible future design if hardware or remote implementations expose materially different requirements. Whether such backend traits would be open is part of ADR-0007.

## Consequences

### Positive

- Devices provide a consistent, capability-oriented public API.
- Generic code can operate on different device types through shared capability traits.
- Mock implementations can support device testing without requiring physical hardware.
- Device structs remain focused on composition and delegation.
- Trait bounds can be limited to the capabilities required by each implementation.
- The same public capability contract can potentially support different hardware mechanisms without changing application-facing device APIs.

### Costs

- Device structs may require multiple generic parameters to represent their composed capabilities.
- Trait implementations can become verbose as devices combine more capabilities.
- Users must import the relevant traits to access their methods. A prelude is deferred until the 0.5.0 API freeze.
- A backend's overridden default method is not automatically invoked when the corresponding default method is called on a device; the device's trait default uses the device's own primitive methods.
- Forwarding default methods to a backend's native implementation may be considered in Stage 2 if a backend offers one with a real benefit (ADR-0009). Testing it needs crate-internal test doubles (ADR-0011). Adopting it would be recorded in a new ADR.
- The type system does not distinguish a device from a backend. A device can therefore be used as a component of another device when it implements the required capability. For example, a `Fan` can satisfy a type parameter of a `Thermostat` that is bounded by `PowerControl`. In that case, calls are delegated through the fan's own implementation to its backend. This composition is not currently required by the crate.
- Public capability traits and backend-facing implementation requirements are not yet separated. Real hardware may demonstrate that their responsibilities need different interfaces.
- Pure delegation is an assumption of this design. A device that requires additional behavioral rules, such as a safety interlock or minimum off time, may need device-level logic beyond delegation. Such a requirement would reopen this decision and be recorded in a new ADR.

## Evidence Still Missing

Stage 2 and later work must establish:

- Whether real GPIO, ESP32, MQTT, and other backends can fit the current composition model cleanly.
- Whether backend-specific requirements justify separating public capability contracts from backend-facing interfaces.
- Whether real devices require behavior beyond composition and delegation.
- Whether forwarding default methods to backends provides the expected semantics.
- Whether crate-internal test doubles can validate backend forwarding behavior (ADR-0011).
- Whether a future device grouping and other trait-object consumers work cleanly with the current capability and device model.
