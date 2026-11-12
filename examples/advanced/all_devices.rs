//! All nine devices, used through the capability traits.
//!
//! One loop drives every device through `PowerControl`. `SpeedControl` and
//! `ClimateControl` each span three device types, so their loops do not
//! depend on the concrete device types. This works because the capability
//! traits are dyn compatible. Brightness, media and position belong to one
//! device type each, so the example calls those on the concrete devices.
//! Run it with `cargo run --example advanced_all_devices`.

use ind_io::mock::{MockClimate, MockLight, MockMedia, MockPosition, MockPower, MockSpeed};
use ind_io::traits::{
    ClimateControl, LightControl, MediaControl, PositionControl, PowerControl, SpeedControl,
};
use ind_io::{
    AirConditioner, AirPurifier, ClimateMode, Curtain, DeviceError, Fan, Heater, Light, Speed,
    Switch, Television, Thermostat,
};

fn main() -> Result<(), DeviceError> {
    let mut switch = Switch::new(MockPower::new());
    let mut fan = Fan::new(MockPower::new(), MockSpeed::new());
    let mut light = Light::new(MockPower::new(), MockLight::new());
    let mut television = Television::new(MockPower::new(), MockMedia::new());
    let mut air_conditioner =
        AirConditioner::new(MockPower::new(), MockSpeed::new(), MockClimate::new());
    let mut thermostat = Thermostat::new(MockPower::new(), MockClimate::new());
    let mut air_purifier = AirPurifier::new(MockPower::new(), MockSpeed::new());
    let mut heater = Heater::new(MockPower::new(), MockClimate::new());
    let mut curtain = Curtain::new(MockPower::new(), MockPosition::new());

    // Power: all nine device types implement `PowerControl`, so one loop
    // covers them. After toggle the device is off, and reset ends with it on.
    let powered: [(&str, &mut dyn PowerControl); 9] = [
        ("switch", &mut switch),
        ("fan", &mut fan),
        ("light", &mut light),
        ("television", &mut television),
        ("air_conditioner", &mut air_conditioner),
        ("thermostat", &mut thermostat),
        ("air_purifier", &mut air_purifier),
        ("heater", &mut heater),
        ("curtain", &mut curtain),
    ];
    for (name, device) in powered {
        device.turn_on()?;
        device.toggle()?;
        let on_after_toggle = device.is_on();
        device.reset()?;
        println!(
            "{name}: on after toggle={on_after_toggle}, on after reset={}",
            device.is_on()
        );
        assert!(!on_after_toggle);
        assert!(device.is_on());
    }

    // Speed: three different devices share one capability.
    let with_speed: [(&str, &mut dyn SpeedControl); 3] = [
        ("fan", &mut fan),
        ("air_conditioner", &mut air_conditioner),
        ("air_purifier", &mut air_purifier),
    ];
    for (name, device) in with_speed {
        device.set_speed(Speed::Medium)?;
        println!("{name}: speed={:?}", device.speed());
        assert_eq!(device.speed(), Speed::Medium);
    }

    // Climate: another capability shared by three devices.
    let with_climate: [(&str, &mut dyn ClimateControl); 3] = [
        ("air_conditioner", &mut air_conditioner),
        ("thermostat", &mut thermostat),
        ("heater", &mut heater),
    ];
    for (name, device) in with_climate {
        device.set_mode(ClimateMode::Heat)?;
        device.set_temperature(21.5)?;
        println!(
            "{name}: mode={:?}, {:.1} C",
            device.mode(),
            device.temperature()
        );
        assert_eq!(device.mode(), ClimateMode::Heat);
        assert_eq!(device.temperature(), 21.5);
    }

    // Brightness, media and position belong to one device type each.
    light.set_brightness(60)?;
    television.set_channel(5)?;
    curtain.set_position(30)?;
    println!(
        "light: {}%, television: channel {}, curtain: {}%",
        light.brightness(),
        television.channel(),
        curtain.position()
    );
    assert_eq!(light.brightness(), 60);
    assert_eq!(television.channel(), 5);
    assert_eq!(curtain.position(), 30);

    Ok(())
}
