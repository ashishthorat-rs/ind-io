//! A curtain combines two capabilities: power and position.
//!
//! The example moves the curtain through a made-up daily schedule, in steps
//! of at most 25 percent. The movement helper is generic over
//! `PositionControl`, so it does not depend on the concrete device type. The
//! schedule and the stepping are application logic: ind-io provides the
//! capability to set and read the position. The helper rejects a zero step
//! with `DeviceError::InvalidInput`. The schedule is simulated, so the
//! example does not wait for real time. Turning the curtain off keeps its
//! position. Run it with `cargo run --example advanced_curtain`.

use ind_io::mock::{MockPosition, MockPower};
use ind_io::traits::{PositionControl, PowerControl};
use ind_io::{Curtain, DeviceError};

/// Moves a device towards a target in steps of at most `step` percent.
///
/// Returns the number of position commands sent. A zero step is rejected
/// with `DeviceError::InvalidInput`.
fn move_in_steps<C: PositionControl>(
    device: &mut C,
    target: u8,
    step: u8,
) -> Result<u32, DeviceError> {
    if step == 0 {
        return Err(DeviceError::InvalidInput);
    }

    let mut current = device.position();
    let mut moves = 0;

    while current != target {
        current = if current < target {
            current.saturating_add(step).min(target)
        } else {
            current.saturating_sub(step).max(target)
        };

        device.set_position(current)?;
        moves += 1;
    }

    Ok(moves)
}

fn main() -> Result<(), DeviceError> {
    let mut curtain = Curtain::new(MockPower::new(), MockPosition::new());

    curtain.turn_on()?;

    // Made-up daily schedule: hour and target position (0-100%). The loop
    // runs through the entries at once, with no clock and no waiting.
    let schedule = [(7, 100), (12, 60), (18, 30), (21, 0)];

    for (hour, target) in schedule {
        let moves = move_in_steps(&mut curtain, target, 25)?;

        println!(
            "{hour:02}:00 -> target {target}%, position {}%, moves {moves}",
            curtain.position()
        );

        assert_eq!(curtain.position(), target);
    }

    // A zero step is rejected and the position stays where it was.
    let position_before_invalid_step = curtain.position();

    assert!(matches!(
        move_in_steps(&mut curtain, 50, 0),
        Err(DeviceError::InvalidInput)
    ));
    assert_eq!(curtain.position(), position_before_invalid_step);

    // Position and power are independent: turning the curtain off keeps the
    // position.
    let position_before_power_off = curtain.position();

    curtain.turn_off()?;

    println!("power off -> position {}%", curtain.position());

    assert!(!curtain.is_on());
    assert_eq!(curtain.position(), position_before_power_off);

    Ok(())
}
