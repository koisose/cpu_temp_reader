# CPU Temperature and Battery Monitor 🌡️ 🔋

A Rust program that monitors your CPU temperature and laptop battery status.

## Features

- 🌡️ CPU Temperature monitoring
- 🔋 Battery status (charge percentage, state)
- ⚡ Power information (voltage, charging time)
- 🌡️ Battery temperature (if available)

## Prerequisites

- Rust and Cargo (Install from [rustup.rs](https://rustup.rs))
- Linux-based system (for temperature sensors)
- Appropriate permissions for reading system information

## Installation

1. Clone the repository:
```bash
git clone <your-repository-url>
cd cpu_temp_reader
```

2. Build the program:
```bash
cargo build --release
```

## Running the Program

### Basic Usage
```bash
cargo run
```

### With sudo (if needed for temperature sensors)
```bash
sudo ~/.cargo/bin/cargo run
```

## Output Example

The program will display information like:
```
Reading CPU temperature...
CPU Temperature: 45.0°C
CPU Usage: 2.5%

-----------------

Battery Status:
Charge: 75.5%
State: Discharging
Time to empty: 3.2 hours
Voltage: 12.1V
Temperature: 30.5°C
```

## Troubleshooting

1. If you can't read CPU temperature:
   - Try running with sudo
   - Check if your system's temperature sensors are properly configured
   - Run `sensors-detect` if needed

2. If battery information is unavailable:
   - Verify that your system has a battery
   - Check if the battery information is accessible in `/sys/class/power_supply/`

## Contributing

Feel free to open issues or submit pull requests if you have suggestions for improvements!

## License

This project is licensed under the MIT License - see the LICENSE file for details.
