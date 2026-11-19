//! A building floor's electrical loads, kept within a power budget.
//!
//! `Floor` is a user-defined struct that owns four ind-io devices built from
//! mock backends: a heater, an air purifier, a ventilation fan and lighting.
//! Each device has a made-up nominal power and a priority. ind-io does not
//! measure or limit power, so the budget and the estimate are application
//! logic.
//!
//! When the estimated load is over the budget, the floor switches loads off,
//! lowest priority first. When the budget allows, it switches back on only
//! the loads it switched off itself, highest priority first, and a load that
//! does not fit holds back the lower-priority ones. A load that the floor did
//! not shed is never restored by the floor. If someone switches a floor-shed
//! load back on manually, the next budget change treats it like any other
//! load that is on. The floor tracks the reported power state, not who last
//! changed it. The devices retain their own settings while off, so a restored
//! load comes back with its previous configuration.
//!
//! The shedding code receives the devices as `&mut dyn PowerControl`. The
//! example does not model a real demand-limiting system. The nominal loads
//! are illustrative estimates, not measured electrical consumption.
//!
//! Run it with `cargo run --example mep_building_floor`.

use ind_io::mock::{MockClimate, MockLight, MockPower, MockSpeed};
use ind_io::traits::{ClimateControl, LightControl, PowerControl, SpeedControl};
use ind_io::{AirPurifier, ClimateMode, DeviceError, Fan, Heater, Light, Speed};

/// Number of loads on the floor.
const LOAD_COUNT: usize = 4;

/// A device with a made-up nominal power.
struct Load<'a> {
    name: &'static str,
    watts: u32,
    device: &'a mut dyn PowerControl,
}

impl<'a> Load<'a> {
    fn new(name: &'static str, watts: u32, device: &'a mut dyn PowerControl) -> Self {
        Self {
            name,
            watts,
            device,
        }
    }
}

/// The loads that one budget change switched off and on.
struct Changes {
    shed: Vec<&'static str>,
    restored: Vec<&'static str>,
}

struct Floor {
    heater: Heater<MockPower, MockClimate>,
    air_purifier: AirPurifier<MockPower, MockSpeed>,
    ventilation: Fan<MockPower, MockSpeed>,
    lighting: Light<MockPower, MockLight>,
    /// Whether the floor switched each load off, in `loads` order.
    shed: [bool; LOAD_COUNT],
}

impl Floor {
    fn new() -> Self {
        Self {
            heater: Heater::new(MockPower::new(), MockClimate::new()),
            air_purifier: AirPurifier::new(MockPower::new(), MockSpeed::new()),
            ventilation: Fan::new(MockPower::new(), MockSpeed::new()),
            lighting: Light::new(MockPower::new(), MockLight::new()),
            shed: [false; LOAD_COUNT],
        }
    }

    /// Switches every device on, then configures it.
    fn start_up(&mut self) -> Result<(), DeviceError> {
        self.heater.turn_on()?;
        self.heater.set_mode(ClimateMode::Heat)?;
        self.heater.set_temperature(21.5)?;

        self.air_purifier.turn_on()?;
        self.air_purifier.set_speed(Speed::Low)?;

        self.ventilation.turn_on()?;
        self.ventilation.set_speed(Speed::Medium)?;

        self.lighting.turn_on()?;
        self.lighting.set_brightness(80)?;

        Ok(())
    }

    /// The loads in shedding order, lowest priority first, and the shed
    /// flags in the same order.
    fn loads(&mut self) -> ([Load<'_>; LOAD_COUNT], &mut [bool; LOAD_COUNT]) {
        let Self {
            heater,
            air_purifier,
            ventilation,
            lighting,
            shed,
        } = self;

        (
            [
                Load::new("heater", 1_500, heater),
                Load::new("air purifier", 150, air_purifier),
                Load::new("ventilation", 600, ventilation),
                Load::new("lighting", 400, lighting),
            ],
            shed,
        )
    }

    /// Sheds or restores loads so that the estimate fits the budget.
    fn set_budget(&mut self, budget_watts: u32) -> Result<Changes, DeviceError> {
        let (mut loads, shed) = self.loads();

        let changes = Changes {
            shed: shed_loads(&mut loads, shed, budget_watts)?,
            restored: restore_loads(&mut loads, shed, budget_watts)?,
        };

        println!(
            "Budget {budget_watts} W: shed {:?}, restored {:?}, estimated load {} W",
            changes.shed,
            changes.restored,
            total_watts(&loads)
        );

        Ok(changes)
    }

    fn estimated_watts(&mut self) -> u32 {
        let (loads, _) = self.loads();
        total_watts(&loads)
    }

    /// The power state of each load, in `loads` order.
    fn power_states(&mut self) -> [bool; LOAD_COUNT] {
        let (loads, _) = self.loads();
        loads.map(|load| load.device.is_on())
    }
}

/// The estimated load of the devices that are on, in watts.
fn total_watts(loads: &[Load<'_>]) -> u32 {
    loads
        .iter()
        .filter(|load| load.device.is_on())
        .map(|load| load.watts)
        .sum()
}

/// Switches loads off, lowest priority first, until the estimate fits.
///
/// Returns the names of the loads it switched off.
fn shed_loads(
    loads: &mut [Load<'_>],
    shed: &mut [bool],
    budget_watts: u32,
) -> Result<Vec<&'static str>, DeviceError> {
    let mut total = total_watts(loads);
    let mut switched_off = Vec::new();

    for (load, was_shed) in loads.iter_mut().zip(shed.iter_mut()) {
        if total <= budget_watts {
            break;
        }

        if load.device.is_on() {
            load.device.turn_off()?;
            *was_shed = true;
            total -= load.watts;
            switched_off.push(load.name);
        }
    }

    Ok(switched_off)
}

/// Switches back on the loads the floor shed, highest priority first.
///
/// Before restoration, reconciles the shed flags with the devices' reported
/// power states. A floor-shed load that is already on is treated as manually
/// restored and is no longer tracked as shed.
///
/// A load that does not fit stops the restoring, so a lower-priority load
/// never goes ahead of it.
///
/// Returns the names of the loads it switched on.
fn restore_loads(
    loads: &mut [Load<'_>],
    shed: &mut [bool],
    budget_watts: u32,
) -> Result<Vec<&'static str>, DeviceError> {
    let mut total = total_watts(loads);
    let mut switched_on = Vec::new();

    // Reconcile every tracked load before priority-based restoration.
    // This must happen before the loop because a later `break` could
    // otherwise leave a manually restored load marked as floor-shed.
    for (load, was_shed) in loads.iter().zip(shed.iter_mut()) {
        if *was_shed && load.device.is_on() {
            *was_shed = false;
        }
    }

    for (load, was_shed) in loads.iter_mut().zip(shed.iter_mut()).rev() {
        if !*was_shed {
            continue;
        }

        if total + load.watts > budget_watts {
            break;
        }

        load.device.turn_on()?;
        *was_shed = false;
        total += load.watts;
        switched_on.push(load.name);
    }

    Ok(switched_on)
}

/// Verifies that a manually restored load is no longer tracked as floor-shed,
/// even when a higher-priority load prevents restoration from reaching it.
fn verify_manual_restoration_tracking() -> Result<(), DeviceError> {
    println!("Manual switch-on while restoring stops early");

    let mut floor = Floor::new();
    floor.start_up()?;

    floor.set_budget(2_000)?;
    floor.set_budget(1_000)?;
    floor.set_budget(500)?;

    // The heater was shed by the floor. Someone switches it back on.
    floor.heater.turn_on()?;
    assert_eq!(floor.power_states(), [true, false, false, true]);
    assert_eq!(floor.estimated_watts(), 1_900);

    // Ventilation does not fit. Restoration stops before reaching the
    // heater, but its shed flag must still be cleared by the pre-pass.
    let changes = floor.set_budget(2_000)?;
    assert!(changes.shed.is_empty());
    assert!(changes.restored.is_empty());
    assert_eq!(floor.power_states(), [true, false, false, true]);

    // The heater is now switched off manually. Since the floor no longer
    // tracks it as shed, a later budget increase must not restore it.
    floor.heater.turn_off()?;

    let changes = floor.set_budget(3_000)?;
    assert!(changes.shed.is_empty());
    assert_eq!(changes.restored, ["ventilation", "air purifier"]);
    assert_eq!(floor.power_states(), [false, true, true, true]);
    assert_eq!(floor.estimated_watts(), 1_150);

    Ok(())
}

fn main() -> Result<(), DeviceError> {
    verify_manual_restoration_tracking()?;

    println!("\nBudget sequence");
    let mut floor = Floor::new();
    floor.start_up()?;

    // The power states are listed as heater, air purifier, ventilation and
    // lighting.
    assert_eq!(floor.power_states(), [true, true, true, true]);
    assert_eq!(floor.estimated_watts(), 2_650);

    // The heater has the lowest priority, so it is shed first.
    let changes = floor.set_budget(2_000)?;
    assert_eq!(changes.shed, ["heater"]);
    assert!(changes.restored.is_empty());
    assert_eq!(floor.power_states(), [false, true, true, true]);
    assert_eq!(floor.estimated_watts(), 1_150);

    // The air purifier is shed next, bringing the estimate to 1,000 W.
    let changes = floor.set_budget(1_000)?;
    assert_eq!(changes.shed, ["air purifier"]);
    assert!(changes.restored.is_empty());
    assert_eq!(floor.power_states(), [false, false, true, true]);
    assert_eq!(floor.estimated_watts(), 1_000);

    let changes = floor.set_budget(500)?;
    assert_eq!(changes.shed, ["ventilation"]);
    assert!(changes.restored.is_empty());
    assert_eq!(floor.power_states(), [false, false, false, true]);
    assert_eq!(floor.estimated_watts(), 400);

    // The ventilation (600 W) does not fit, and the air purifier (150 W)
    // would fit but has a lower priority, so it stays off too.
    let changes = floor.set_budget(600)?;
    assert!(changes.shed.is_empty());
    assert!(changes.restored.is_empty());
    assert_eq!(floor.power_states(), [false, false, false, true]);
    assert_eq!(floor.estimated_watts(), 400);

    // The ventilation fits exactly, at 1,000 W. The air purifier would make
    // 1,150 W, so it stays off, and the heater waits behind it.
    let changes = floor.set_budget(1_000)?;
    assert!(changes.shed.is_empty());
    assert_eq!(changes.restored, ["ventilation"]);
    assert_eq!(floor.power_states(), [false, false, true, true]);
    assert_eq!(floor.estimated_watts(), 1_000);

    // Someone switches the air purifier back on manually. The floor no
    // longer treats it as shed, and the heater fits at 2,650 W.
    floor.air_purifier.turn_on()?;
    let changes = floor.set_budget(2_700)?;
    assert!(changes.shed.is_empty());
    assert_eq!(changes.restored, ["heater"]);
    assert_eq!(floor.power_states(), [true, true, true, true]);
    assert_eq!(floor.estimated_watts(), 2_650);

    // Someone switches the ventilation off manually. The floor did not
    // shed it, so it does not switch it back on.
    floor.ventilation.turn_off()?;
    let changes = floor.set_budget(3_000)?;
    assert!(changes.shed.is_empty());
    assert!(changes.restored.is_empty());
    assert_eq!(floor.power_states(), [true, true, false, true]);
    assert_eq!(floor.estimated_watts(), 2_050);

    // Every device kept its own settings while it was off.
    assert_eq!(floor.heater.mode(), ClimateMode::Heat);
    assert_eq!(floor.heater.temperature(), 21.5);
    assert_eq!(floor.air_purifier.speed(), Speed::Low);
    assert_eq!(floor.ventilation.speed(), Speed::Medium);
    assert_eq!(floor.lighting.brightness(), 80);

    Ok(())
}
