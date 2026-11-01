# ADR-0005 — Capability Trait Design

**Status:** Accepted (Provisional)

**Date:** 2026-09-28

**Recorded:** 2026-09-30 — `feat(traits): add capability module structure` and the six trait commits

**Amended:**

- 2026-10-04 — explicit mute (`mute`, `unmute`, `is_muted`, with `toggle_mute` as a default method) replaced a bare toggle: `refactor(traits): make MediaControl mute state explicit`.

**Stage:** 1

**Review Trigger:** First real backend (Stage 2), or earlier if a capability boundary proves incorrect.

## Context

`ind-io` provides a hardware-agnostic abstraction for controlling IoT and building-automation devices.

Different devices expose different combinations of functionality. A fan may provide power and speed control, while an air conditioner may provide power, speed, and climate control. A television may provide power and media control, while a curtain may provide power and position control.

The API therefore needs reusable capability contracts rather than a separate collection of operations for every physical device type.

The main design question is how these capabilities should be divided into public traits. The traits need to be small enough to remain reusable, but grouped enough to avoid an unnecessarily large vocabulary of one-method traits.

## Decision

`ind-io` defines six public capability traits (sealed; see ADR-0007) for Stage 1:

- `PowerControl`
- `SpeedControl`
- `ClimateControl`
- `MediaControl`
- `LightControl`
- `PositionControl`

Each trait groups the operations that belong to one capability, instead of offering one trait per operation. `PowerControl` covers switching and the power state, and `SpeedControl` covers a discrete speed setting. `ClimateControl` covers temperature, mode, and swing. `MediaControl` covers channel, volume, and mute. `LightControl` covers brightness. `PositionControl` covers a 0 to 100 percent position, with `open` and `close` operations.

`PositionControl` is intentionally a general capability rather than a curtain-specific abstraction. This allows devices such as curtains, modulating valves, and dampers to share the same position-control semantics where their behavior is genuinely equivalent.

Mute is explicit: `mute`, `unmute` and `is_muted`, with `toggle_mute` as a default method, instead of a bare toggle or a volume of zero. Hardware that exposes only a mute toggle can still be driven, because the backend can read `is_muted` and send the toggle only when the state differs.

Volume step size is backend-defined because the appropriate increment depends on the controlled device.

The six traits are independent at the type level: none requires another, so a device implements only the capabilities it provides. How their values interact at run time is decided in ADR-0008.

The traits correspond conceptually to Matter clusters: on/off, fan, thermostat, level and window covering. Known differences are that Matter keeps swing in a different cluster from the thermostat and uses levels from 0 to 254, so a Matter backend would convert between representations. These details are to be confirmed when a Matter backend is attempted.

How devices expose and compose these capabilities is defined separately in ADR-0012 — Devices Implement Capability Traits.

## Alternatives Considered

### Device-specific traits

A separate trait could be defined for each device type, such as fan, heater, or air conditioner control.

**Rejected.** Common capabilities would be duplicated across device types, making reuse and generic capability-based control more difficult.

### Single universal device trait

A single trait could contain power, speed, temperature, media, lighting, position, and other operations.

**Rejected.** Devices would be associated with operations they do not support, creating an increasingly broad and weak semantic contract as new capabilities are added.

### Atomic traits — one per operation

A trait per operation, such as `SetTemperature`, `SetMode`, and `SetSwing`, would allow a backend to implement exactly the operations it supports.

**Rejected.** A device would need a bound for every operation, generic code would repeat long lists of bounds, and the public vocabulary would grow with every method. The cost of grouping is accepted: an implementation may return `DeviceError::NotSupported` for an individual operation when the underlying hardware does not provide that functionality.

## Consequences

### Positive

- Shared capabilities can be reused across multiple device types.
- Devices can expose only the capabilities relevant to their functionality.
- Generic control code can operate on capabilities rather than concrete device types.
- New device types can reuse existing capabilities without requiring a new trait for every device.
- The same capability model can potentially serve consumer IoT and industrial equipment when their control semantics are equivalent.
- The public vocabulary remains smaller than an operation-per-trait design.

### Costs

- A backend must implement every method of a grouped trait. One that lacks an operation, such as a thermostat without swing, returns `DeviceError::NotSupported` for it.
- The traits describe configured state only. Stage 1 does not distinguish measured environmental values from configured target values. More advanced HVAC control may require separate measurement, setpoint, and actuator-feedback interfaces; these are outside the current capability contracts. This limitation is recorded in ADR-0006.
- Splitting or merging a capability trait after release is a breaking public API change.
- Some capabilities may require refinement when real hardware demonstrates that their current boundaries are incorrect.

## Evidence Still Missing

Stage 2 and later work must establish:

- Whether GPIO, ESP32, MQTT, and other real backends can implement the grouped traits without excessive `NotSupported` operations.
- Whether HVAC and industrial equipment require measured values, separate setpoints, or actuator feedback.
- Whether `PositionControl` provides sufficiently general semantics for curtains, valves, dampers, and similar equipment.
- Whether the six capability boundaries remain appropriate when real hardware is integrated.
- Whether the conceptual Matter correspondence remains valid, including the actual cluster mapping and level conversion, when a Matter integration is attempted.
