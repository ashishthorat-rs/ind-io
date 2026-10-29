# Name and Positioning

This document records what `ind-io` is called, what it claims to be, and what it does not claim. It is a project-identity record, not an architecture decision record: the ADRs in this folder cover technical decisions only. The README will give the short public version and links here. CONTRIBUTING covers contributor guidance.

**Decided:** 2026-09-20

**Status:** Name kept, with a wording policy (below).

**Review Trigger:** see the last section.

## Name

The crate name is `ind-io`, and the library name in Rust code is `ind_io` (`use ind_io::...`).

## Meaning

`ind-io` is a name, not an abbreviation of a phrase. It is deliberately left unexpanded. The "ind" leaves room for the industrial direction described below, but it does **not** stand for "Industrial I/O", which is the name of a Linux kernel subsystem (see Naming risks). Documentation must not expand it.

## Why `ind-io`

- It looks ahead. The capability model is designed so that the crate can later reach HVAC, MEP and industrial equipment, and the name is meant to fit that direction without a rename. Stage 1 does not deliver it yet: it provides consumer-style devices with in-memory mock backends. The name states a direction, and the documentation claims only what exists (see Positioning).
- It was chosen on 2026-09-20 and has been the name of the repository and the crate since the first commit.
- It is short enough for a `use` line.
- The name was settled early to keep the crate, repository, import, examples and documentation consistent. Renaming after publication would require coordinated updates and could disrupt existing users and links.

## What `ind-io` Controls

`ind-io` is a hardware-agnostic, `no_std` abstraction for controlling devices through capabilities. In Stage 1 it provides:

- Six sealed capability traits: power, speed, climate, media, light and position control (ADR-0005, ADR-0007);
- Nine devices composed from those capabilities: Switch, Fan, Light, Television, AirConditioner, AirPurifier, Heater, Thermostat and Curtain;
- In-memory mock backends for every capability, used for tests and examples.

The state it exposes is configured state: what was last set on a device, not what a sensor measured (ADR-0006).

## What `ind-io` Does Not Claim

- It is not related to the `industrial-io` crate, which wraps the Linux Industrial I/O (IIO) subsystem for analog hardware such as ADCs and sensors. `ind-io` does no analog I/O, sensor reading or data acquisition.
- It is not related to Industrunio's IND.I/O Arduino-compatible controller or its Indio Library.
- It has no real hardware backend yet. GPIO, ESP32, MQTT and other backends are later stages.
- It does not implement industrial or building protocols such as Modbus or BACnet.
- It does not claim compatibility with Matter. The capability traits correspond to Matter clusters conceptually, and nothing has been tested against Matter (ADR-0005).
- It makes no safety, certification or reliability claim for industrial or safety-critical use.
- It does not yet serve industrial or HVAC equipment in practice. The traits are designed so that it could; real equipment may show that they need measured values and separate setpoints.

## Positioning

Capability-based device control for IoT and building automation, with `no_std` traits and in-memory mock backends.

Wording policy, applied to the README, crate documentation, `Cargo.toml` description and keywords, and the reference document:

1. Never expand "ind-io" as "Industrial I/O".
2. Describe the crate by its capabilities and devices, not by its name.
3. Keep "industrial" and "HVAC" out of the description and keywords until industrial backends exist. The direction may be stated in the README, under a heading that marks it as planned.
4. Keep one line in the README that says it is not the `industrial-io` crate.

## Alternatives Considered

### Keep `ind-io` and add the wording policy

**Chosen.** The overlap exists in prose and not in code or manifest, so wording removes most of it at the lowest cost.

### Rename to a descriptive name

**Not adopted.** A descriptive name, built from capability, device or appliance terms, would help adoption. It would have to be short, unused as a phrase, and free on crates.io and GitHub. The benefit was judged smaller than the rename cost and the risk of choosing a worse name, and it would give up the name's fit with the industrial direction.

### Use the name to claim an industrial focus now

**Not adopted.** The name looks ahead, but Stage 1 delivers no industrial backend. The description and keywords claim only what exists, and the README states the direction as a plan.

## Naming Risks

- **Search confusion.** A search for "industrial io rust" finds the `industrial-io` crate. Readers may assume `ind-io` is a sensor or ADC library. The wording policy and the README disambiguation line are the mitigation.
- **Typo and autocomplete confusion** between `ind-io` and `industrial-io` remains and cannot be removed without a rename.
- **A name that says little.** `ind-io` says nothing about capabilities or devices, and "ind" suggests "industrial", which Stage 1 does not deliver.

## Current Status

- The name is in use as the crate, library and repository name.
- It first appears in the repository in the initial project scaffold commit (2026-09-24) and in the public import path from the first integration tests.
- The overlap with `industrial-io` was reviewed during Stage 1, before the examples were written, and the name was kept.

## Review Trigger

Reopen this document when any of these happens:

- The first real hardware backend lands: update "what `ind-io` controls" and "what `ind-io` does not claim", which describe Stage 1;
- A first industrial backend, such as Modbus or BACnet, is planned: reconsider the wording policy and whether the description and keywords can now name industrial use;
- A review finds that the README's first paragraph reads like the Linux IIO crate, which means the wording policy is not working.
