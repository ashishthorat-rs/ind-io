//! A duty pump station with lead rotation.
//!
//! The crate has no pump type, so the example uses a `Fan`, built from mock
//! backends, for its power and speed capabilities. The speed stands in for a
//! made-up drive speed. The station has two pumps. When there is demand, the
//! pump with fewer run ticks leads, and at peak demand the other pump
//! assists. The lead therefore changes between demand episodes, and the run
//! ticks of the two pumps stay close.
//!
//! The rotation, the run ticks and the demand are application logic over
//! simulated ticks. ind-io does not track run time, measure flow or
//! pressure, or report faults, so the example has no failover. It does not
//! model a real pump station or protect real pumps.
//!
//! Run it with `cargo run --example mep_pump`.

use ind_io::mock::{MockPower, MockSpeed};
use ind_io::traits::{PowerControl, SpeedControl};
use ind_io::{DeviceError, Fan, Speed};

const STOPPED: Option<Speed> = None;
const MEDIUM: Option<Speed> = Some(Speed::Medium);
const HIGH: Option<Speed> = Some(Speed::High);

#[derive(Clone, Copy, Debug)]
enum Demand {
    Idle,
    Normal,
    Peak,
}

struct Pump {
    name: &'static str,
    fan: Fan<MockPower, MockSpeed>,
    run_ticks: u32,
}

impl Pump {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            fan: Fan::new(MockPower::new(), MockSpeed::new()),
            run_ticks: 0,
        }
    }

    /// Runs the pump at `speed`. The pump is switched on before its speed is
    /// set, so the example does not depend on a backend accepting settings
    /// while it is off.
    fn run(&mut self, speed: Speed) -> Result<(), DeviceError> {
        self.fan.turn_on()?;
        self.fan.set_speed(speed)
    }

    fn stop(&mut self) -> Result<(), DeviceError> {
        self.fan.turn_off()
    }

    /// The speed while the pump is running, or `None` while it is stopped.
    fn running_speed(&self) -> Option<Speed> {
        self.fan.is_on().then(|| self.fan.speed())
    }
}

struct Station {
    pumps: [Pump; 2],
    lead: Option<usize>,
}

impl Station {
    fn new() -> Self {
        Self {
            pumps: [Pump::new("A"), Pump::new("B")],
            lead: None,
        }
    }

    /// The pump with fewer run ticks. A tie goes to pump A.
    fn fewest_run_ticks(&self) -> usize {
        usize::from(self.pumps[1].run_ticks < self.pumps[0].run_ticks)
    }

    /// Keeps the lead for the current demand episode, or chooses a new one.
    fn lead(&mut self) -> usize {
        let lead = self.lead.unwrap_or_else(|| self.fewest_run_ticks());

        self.lead = Some(lead);
        lead
    }

    /// Applies the demand for one tick and counts the run ticks.
    fn tick(&mut self, demand: Demand) -> Result<(), DeviceError> {
        match demand {
            Demand::Idle => {
                self.lead = None;
                for pump in &mut self.pumps {
                    pump.stop()?;
                }
            }
            Demand::Normal => {
                let lead = self.lead();
                self.pumps[lead].run(Speed::Medium)?;
                self.pumps[1 - lead].stop()?;
            }
            Demand::Peak => {
                let lead = self.lead();
                self.pumps[lead].run(Speed::High)?;
                self.pumps[1 - lead].run(Speed::Medium)?;
            }
        }

        for pump in &mut self.pumps {
            if pump.fan.is_on() {
                pump.run_ticks += 1;
            }
        }

        Ok(())
    }

    fn speeds(&self) -> [Option<Speed>; 2] {
        [self.pumps[0].running_speed(), self.pumps[1].running_speed()]
    }
}

/// The name of a speed, or "stopped" when the pump is not running.
fn describe(speed: Option<Speed>) -> String {
    speed.map_or_else(|| String::from("stopped"), |speed| format!("{speed:?}"))
}

fn main() -> Result<(), DeviceError> {
    let mut station = Station::new();

    // The demand for each tick, and the speed each pump should show
    // afterwards. The expected speeds are written out by hand from the rules.
    let ticks = [
        (Demand::Idle, [STOPPED, STOPPED]),
        // Episode 1: both pumps have 0 run ticks, so pump A leads.
        (Demand::Normal, [MEDIUM, STOPPED]),
        (Demand::Normal, [MEDIUM, STOPPED]),
        (Demand::Peak, [HIGH, MEDIUM]),
        (Demand::Normal, [MEDIUM, STOPPED]),
        (Demand::Idle, [STOPPED, STOPPED]),
        (Demand::Idle, [STOPPED, STOPPED]),
        // Episode 2: pump A has 4 run ticks and pump B has 1, so B leads.
        (Demand::Normal, [STOPPED, MEDIUM]),
        (Demand::Peak, [MEDIUM, HIGH]),
        (Demand::Peak, [MEDIUM, HIGH]),
        (Demand::Idle, [STOPPED, STOPPED]),
        // Episode 3: pump A has 6 run ticks and pump B has 4, so B leads.
        (Demand::Normal, [STOPPED, MEDIUM]),
        (Demand::Normal, [STOPPED, MEDIUM]),
        (Demand::Normal, [STOPPED, MEDIUM]),
        (Demand::Idle, [STOPPED, STOPPED]),
        // Episode 4: pump A has 6 run ticks and pump B has 7, so A leads.
        (Demand::Normal, [MEDIUM, STOPPED]),
        (Demand::Normal, [MEDIUM, STOPPED]),
        (Demand::Idle, [STOPPED, STOPPED]),
    ];

    for (number, (demand, expected)) in ticks.into_iter().enumerate() {
        station.tick(demand)?;

        let speeds = station.speeds();
        let demand_label = format!("{demand:?}");
        println!(
            "tick {number:2}: {demand_label:<6} A {:<8} B {:<8} run ticks A {}, B {}",
            describe(speeds[0]),
            describe(speeds[1]),
            station.pumps[0].run_ticks,
            station.pumps[1].run_ticks
        );

        assert_eq!(speeds, expected, "tick {number}");
    }

    // The lead rotated by run ticks, so two pumps ended one tick apart.
    assert_eq!(station.pumps[0].run_ticks, 8);
    assert_eq!(station.pumps[1].run_ticks, 7);
    println!(
        "pump {} ran {} ticks, pump {} ran {} ticks",
        station.pumps[0].name,
        station.pumps[0].run_ticks,
        station.pumps[1].name,
        station.pumps[1].run_ticks,
    );

    Ok(())
}
