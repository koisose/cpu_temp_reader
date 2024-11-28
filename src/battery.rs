use battery::{Manager, State};
use battery::units::ratio::ratio;
use battery::units::time::hour;
use battery::units::electric_potential::volt;
use battery::units::thermodynamic_temperature::degree_celsius;
use std::error::Error;

pub fn check_battery() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new()?;
    let batteries = manager.batteries()?;

    for battery in batteries {
        let battery = battery?;
        
        // Get battery percentage
        let percentage = battery.state_of_charge().get::<ratio>() * 100.0;
        
        // Get charging state
        let state = match battery.state() {
            State::Charging => "Charging",
            State::Discharging => "Discharging",
            State::Empty => "Empty",
            State::Full => "Full",
            _ => "Unknown",
        };
        
        // Get time to full/empty if available
        let time_to_full = battery.time_to_full();
        let time_to_empty = battery.time_to_empty();
        
        // Print battery information
        println!("Battery Status:");
        println!("Charge: {:.1}%", percentage);
        println!("State: {}", state);
        
        if let Some(time) = time_to_full {
            println!("Time to full: {:.1} hours", time.get::<hour>());
        }
        
        if let Some(time) = time_to_empty {
            println!("Time to empty: {:.1} hours", time.get::<hour>());
        }
        
        // Print voltage and temperature if available
        println!("Voltage: {:.2}V", battery.voltage().get::<volt>());
        if let Some(temp) = battery.temperature() {
            println!("Temperature: {:.1}°C", temp.get::<degree_celsius>());
        }
    }
    
    Ok(())
}
