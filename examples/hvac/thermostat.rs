//! Zone thermostats with heat/cool changeover and a night setback.
//!
//! Each zone is a `Thermostat` with two setpoints and a dead band between
//! them: below the heating setpoint the zone heats, above the cooling
//! setpoint it cools, and in between it is switched off. The application
//! chooses the setpoints. It selects the occupied or night profile and
//! applies it to each zone. The same set of room readings can therefore
//! produce different demands under the two profiles, with each zone
//! evaluated using its own reading. The room readings are made up, because
//! ind-io does not read sensors or provide a control strategy, and the
//! backends are mocks.
//!
//! Run it with `cargo run --example hvac_thermostat`.

use ind_io::mock::{MockClimate, MockPower};
use ind_io::traits::{ClimateControl, PowerControl};
use ind_io::{ClimateMode, DeviceError, Thermostat};

/// Heating and cooling setpoints, in degrees Celsius.
struct Setpoints {
    heat: f32,
    cool: f32,
}

/// Setpoints while the building is occupied.
const COMFORT: Setpoints = Setpoints {
    heat: 21.0,
    cool: 24.0,
};

/// Wider setpoints while the building is unoccupied at night.
const SETBACK: Setpoints = Setpoints {
    heat: 16.0,
    cool: 28.0,
};

#[derive(Debug, PartialEq)]
enum Demand {
    Heating,
    Cooling,
    Idle,
}

struct Zone {
    name: &'static str,
    room_celsius: f32,
    thermostat: Thermostat<MockPower, MockClimate>,
}

impl Zone {
    fn new(name: &'static str, room_celsius: f32) -> Self {
        Self {
            name,
            room_celsius,
            thermostat: Thermostat::new(MockPower::new(), MockClimate::new()),
        }
    }

    fn demand(&self) -> Demand {
        match (self.thermostat.is_on(), self.thermostat.mode()) {
            (true, ClimateMode::Heat) => Demand::Heating,
            (true, ClimateMode::Cool) => Demand::Cooling,
            _ => Demand::Idle,
        }
    }
}

/// Heats below `heat`, cools above `cool` and switches off in between.
///
/// A zone is switched on before its mode and target are set, so the
/// operation does not depend on a backend accepting settings while it is
/// off. The operations are sequential, not atomic: if a later operation
/// fails, an earlier successful change is not rolled back.
fn changeover(
    thermostat: &mut (impl PowerControl + ClimateControl),
    room_celsius: f32,
    setpoints: &Setpoints,
) -> Result<(), DeviceError> {
    if room_celsius < setpoints.heat {
        thermostat.turn_on()?;
        thermostat.set_mode(ClimateMode::Heat)?;
        thermostat.set_temperature(setpoints.heat)
    } else if room_celsius > setpoints.cool {
        thermostat.turn_on()?;
        thermostat.set_mode(ClimateMode::Cool)?;
        thermostat.set_temperature(setpoints.cool)
    } else {
        thermostat.turn_off()
    }
}

/// Applies one setpoint profile independently to every zone.
fn run(label: &str, zones: &mut [Zone], setpoints: &Setpoints) -> Result<(), DeviceError> {
    println!(
        "{label} (heat below {:.0} C, cool above {:.0} C)",
        setpoints.heat, setpoints.cool
    );

    for zone in zones.iter_mut() {
        changeover(&mut zone.thermostat, zone.room_celsius, setpoints)?;

        let state = match zone.demand() {
            Demand::Idle => String::from("Idle"),
            demand => format!("{demand:?} to {:.1} C", zone.thermostat.temperature()),
        };

        println!(
            "  {:<10} room {:4.1} C -> {state}",
            zone.name, zone.room_celsius
        );
    }

    Ok(())
}

fn main() -> Result<(), DeviceError> {
    let mut zones = [
        Zone::new("office", 19.5),
        Zone::new("south wing", 25.5),
        Zone::new("lobby", 22.5),
        Zone::new("storage", 15.0),
    ];

    // Apply occupied-period setpoints.
    run("occupied", &mut zones, &COMFORT)?;

    assert_eq!(zones[0].demand(), Demand::Heating);
    assert_eq!(zones[0].thermostat.temperature(), 21.0);

    assert_eq!(zones[1].demand(), Demand::Cooling);
    assert_eq!(zones[1].thermostat.temperature(), 24.0);

    assert_eq!(zones[2].demand(), Demand::Idle);
    assert!(!zones[2].thermostat.is_on());

    assert_eq!(zones[3].demand(), Demand::Heating);
    assert_eq!(zones[3].thermostat.temperature(), 21.0);

    // The same readings with wider night setpoints: only the storage zone,
    // which is below 16 C, continues heating.
    run("night setback", &mut zones, &SETBACK)?;

    assert_eq!(zones[0].demand(), Demand::Idle);
    assert_eq!(zones[1].demand(), Demand::Idle);
    assert_eq!(zones[2].demand(), Demand::Idle);

    assert_eq!(zones[3].demand(), Demand::Heating);
    assert_eq!(zones[3].thermostat.temperature(), 16.0);

    // The zones that became idle are switched off, but retain their
    // previously configured climate mode and target.
    assert!(!zones[0].thermostat.is_on());
    assert_eq!(zones[0].thermostat.mode(), ClimateMode::Heat);
    assert_eq!(zones[0].thermostat.temperature(), 21.0);

    assert!(!zones[1].thermostat.is_on());
    assert_eq!(zones[1].thermostat.mode(), ClimateMode::Cool);
    assert_eq!(zones[1].thermostat.temperature(), 24.0);

    assert!(!zones[2].thermostat.is_on());

    Ok(())
}
