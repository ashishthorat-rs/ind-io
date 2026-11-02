# ADR-0008 — Capabilities Are Independent

**Status:** Accepted (Provisional)

**Date:** 2026-09-28

**Recorded:** 2026-09-30 — `feat(traits): add PowerControl capability`

**Amended:**

- 2026-10-04 — `refactor(traits): make MediaControl mute state explicit`: mute becomes a state of its own, independent of the volume level.
- 2026-10-06 — the mocks apply the rule: `feat(mock): add MockLight backend` (brightness 0 is a level) and `feat(mock): add MockMedia backend` (mute is independent of volume).
- 2026-10-07 — `feat(mock): add MockPosition backend` and `feat(mock): add MockSpeed backend` (position 0 and `Speed::Off` are not power states).
- 2026-10-12 — `test(air_conditioner): add AirConditioner integration tests`: the rules are enforced at the device level, and `reset` is pinned as a power cycle that ends on with settings unchanged.
- 2026-10-17 — `test(air_conditioner): assert swing is kept across power changes`.

**Stage:** 1

**Review Trigger:** First real hardware backend in Stage 2, or earlier if a device requires coupling between capabilities.

## Context

ADR-0005 makes the capability traits independent at the type level. This ADR decides how their values interact at run time.

Devices combine capabilities such as power, speed, climate, media, light, and position. Some values may appear to imply the state of another capability. For example, a brightness of zero, a position of zero, `Speed::Off`, or a volume of zero might be interpreted as meaning that a device is powered off.

Climate settings can also be incorrectly coupled to power state. Setting a temperature, changing a climate mode, or changing a swing mode should configure the climate capability without implicitly turning the device on or off.

Similarly, muting a television does not necessarily change its volume level. A device may remain powered on while muted or while its volume is zero.

Inferring one capability's state from another can introduce hidden side effects, make generic behavior unpredictable, and reduce the ability to compose capabilities across different device types.

## Decision

Each capability represents an independent semantic concern. An operation within one capability must not implicitly change another capability's state unless the relevant API contract explicitly defines that behavior. No Stage 1 contract defines such coupling, so this exception is currently unused.

Power state changes only through `PowerControl`.

A brightness of 0, a position of 0, `Speed::Off` and a volume of 0 are levels of their own capability, not power states. A device can be on at any of them.

Within that rule, the following hold for every device:

- When a setting operation is accepted, it changes only the relevant capability's configured value and does not change the device's power state.
- Turning a device on or off preserves its configured capability values.
- Mute and volume are independent in both directions: muting keeps the volume level, and changing the volume does not change the mute state.
- `reset` is a power cycle. It turns the device off and then on, and it ends on. Like any power change, it preserves configured settings. It is not a factory reset.

These rules define semantic independence, not a guarantee that every physical device can execute every operation in every state. A backend may reject an operation that its hardware cannot support in the current state, but it must not silently change an unrelated capability to simulate success. A backend that cannot preserve configured settings across a power change must report an error through `DeviceError` rather than report success.

## Alternatives Considered

### Implicit capability coupling

Infer power state from values such as zero brightness, zero position, or `Speed::Off`, or automatically change power state when another capability is modified.

**Rejected.** These inferences introduce hidden side effects and make behavior unpredictable across devices.

### Device-specific coupling rules

Allow each device type to define its own implicit relationships between capabilities.

**Not adopted in Stage 1.** This would make the same capability operation behave differently across device types and introduce policy into device composition. Reconsider only if real hardware demonstrates a necessary relationship that cannot be represented adequately by independent capability contracts.

### Explicit coupled operations

A coupled operation could be offered as its own method, such as turning on at a given brightness.

**Not adopted in Stage 1.** It can be added later as a new method without breaking independence, because the coupling would be explicit in the method's contract. Matter's Level Control cluster has separate commands for changing a level alone (`MoveToLevel`) and for changing it together with the On/Off state (`MoveToLevelWithOnOff`).

## Consequences

### Positive

- Capability operations have predictable semantics and fewer hidden side effects.
- Power state remains explicit and is controlled through `PowerControl`.
- Generic code can work with different device types without assuming that one capability determines another.
- Applications retain control over automation and coordination policy.
- Device composition remains consistent with the capability contracts defined in ADR-0005 and ADR-0012.

### Costs

- Applications must explicitly perform multiple operations when their intended behavior requires coordinating capabilities.
- A powered-on device may legitimately have `Speed::Off`, zero brightness, zero volume, or a position of zero.
- Hardware that cannot accept a command in its current state may require the backend to return an appropriate error.
- Retention across power transitions is implemented by the mocks and checked by the device tests. Whether real hardware can honor it has not been validated.

## Evidence Still Missing

Stage 2 and later work must establish:

- Whether real devices can accept and retain configuration changes while powered off.
- Whether any supported hardware necessarily couples power state to speed, brightness, position, or another capability.
- Whether power transitions preserve configured capability values across the local and remote backends planned for later stages.
- Whether the capability independence rules remain practical for real HVAC equipment and other devices with hardware-enforced constraints.
