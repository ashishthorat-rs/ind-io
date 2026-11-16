//! A chiller staged by return-water temperature, with a restart lockout.
//!
//! The crate has no chiller type, so the example uses an `AirConditioner`,
//! the same type that can control a bedroom air conditioner, with mock
//! backends. It uses the power, speed and climate capabilities to
//! demonstrate application-level chiller logic and does not model real
//! equipment. The speed capability is mapped to illustrative capacity
//! stages, and the climate capability sets a chilled-water supply target
//! of 7 C.
//!
//! `Speed::Off` means the chiller is unloaded, not stopped. The chiller
//! remains powered on while its capacity stage is Off.
//!
//! The restart lockout is application logic over simulated ticks. It
//! demonstrates a minimum wait between stop and restart requests; it is
//! not a real-time timer or a compressor protection mechanism.
//!
//! The temperatures, stages and lockout duration are illustrative, not
//! engineering recommendations. ind-io does not measure water temperature
//! or load, and it does not provide a chiller control strategy.
//!
//! Run it with `cargo run --example hvac_chiller`.

use ind_io::mock::{MockClimate, MockPower, MockSpeed};
use ind_io::traits::{ClimateControl, PowerControl, SpeedControl};
use ind_io::{AirConditioner, ClimateMode, DeviceError, Speed};

type Chiller = AirConditioner<MockPower, MockSpeed, MockClimate>;

/// Chilled-water supply setpoint, in degrees Celsius.
const SETPOINT_CELSIUS: f32 = 7.0;

/// Number of simulated ticks required before a stopped chiller may restart.
const RESTART_LOCKOUT_TICKS: u32 = 3;

/// Maps return-water temperature to an illustrative capacity stage.
///
/// The further the return temperature is above the supply setpoint,
/// the higher the requested stage. `Speed::Off` means unloaded,
/// not stopped.
fn stage_for(return_celsius: f32) -> Speed {
    match return_celsius - SETPOINT_CELSIUS {
        difference if difference > 4.0 => Speed::High,
        difference if difference > 2.0 => Speed::Medium,
        difference if difference > 0.5 => Speed::Low,
        _ => Speed::Off,
    }
}

/// Application policy that delays a restart after a stop.
///
/// The counter advances in simulated ticks. It does not measure real time
/// or implement equipment-level compressor protection.
struct RestartGuard {
    ticks_since_stop: u32,
}

impl RestartGuard {
    fn new() -> Self {
        // A chiller that has not been stopped may start at once.
        Self {
            ticks_since_stop: RESTART_LOCKOUT_TICKS,
        }
    }

    fn stopped(&mut self) {
        self.ticks_since_stop = 0;
    }

    fn tick(&mut self) {
        self.ticks_since_stop = self.ticks_since_stop.saturating_add(1);
    }

    fn may_start(&self) -> bool {
        self.ticks_since_stop >= RESTART_LOCKOUT_TICKS
    }
}

/// Starts the chiller unless the application-level lockout is active.
///
/// Returns `Ok(false)` when a restart is refused by the guard.
fn try_start<D: PowerControl>(chiller: &mut D, guard: &RestartGuard) -> Result<bool, DeviceError> {
    if !guard.may_start() {
        return Ok(false);
    }

    chiller.turn_on()?;
    Ok(true)
}

fn main() -> Result<(), DeviceError> {
    let mut chiller = Chiller::new(MockPower::new(), MockSpeed::new(), MockClimate::new());
    let mut guard = RestartGuard::new();

    // Start the chiller before configuring its operating mode and target.
    let started = try_start(&mut chiller, &guard)?;
    assert!(started);

    chiller.set_mode(ClimateMode::Cool)?;
    chiller.set_temperature(SETPOINT_CELSIUS)?;

    // Simulated return-water temperatures and their requested stages.
    let return_temperatures = [12.5, 11.8, 10.0, 8.6, 7.9, 7.3, 7.1, 6.9, 7.2, 8.0, 9.5];

    let mut stages = Vec::new();

    for return_celsius in return_temperatures {
        let stage = stage_for(return_celsius);
        chiller.set_speed(stage)?;

        println!(
            "return {return_celsius:4.1} C -> stage {:?}",
            chiller.speed()
        );

        stages.push(chiller.speed());
    }

    assert_eq!(
        stages,
        [
            Speed::High,
            Speed::High,
            Speed::Medium,
            Speed::Low,
            Speed::Low,
            Speed::Off,
            Speed::Off,
            Speed::Off,
            Speed::Off,
            Speed::Low,
            Speed::Medium,
        ]
    );

    // An unloaded chiller remains powered on.
    assert!(chiller.is_on());

    // Stop the chiller, then attempt a restart during the lockout.
    chiller.turn_off()?;
    guard.stopped();

    for tick in 1..=RESTART_LOCKOUT_TICKS {
        guard.tick();

        let started = try_start(&mut chiller, &guard)?;

        println!(
            "tick {tick}: restart {}",
            if started { "allowed" } else { "refused" }
        );

        assert_eq!(started, tick == RESTART_LOCKOUT_TICKS);
        assert_eq!(chiller.is_on(), started);
    }

    // Stopping and restarting did not erase the configured mode or target.
    assert!(chiller.is_on());
    assert_eq!(chiller.temperature(), SETPOINT_CELSIUS);
    assert_eq!(chiller.mode(), ClimateMode::Cool);

    Ok(())
}
