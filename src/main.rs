use sysinfo::{CpuExt, System, SystemExt, ComponentExt};

mod battery;

fn main() {
    // Create a new System instance
    let mut sys = System::new();

    println!("Reading CPU temperature...");
    
    // Refresh CPU information
    sys.refresh_cpu();
    sys.refresh_components_list();
    
    // Get CPU temperature from components
    if let Some(components) = sys.components().first() {
        println!("CPU Temperature: {:.1}°C", components.temperature());
    } else {
        println!("Could not read CPU temperature. Make sure you have the necessary permissions.");
    }
    
    // Display CPU usage
    let cpu_usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
    println!("CPU Usage: {:.1}%", cpu_usage);
    
    println!("\n-----------------\n");
    
    // Check battery status
    if let Err(e) = battery::check_battery() {
        println!("Error reading battery information: {}", e);
    }
}
