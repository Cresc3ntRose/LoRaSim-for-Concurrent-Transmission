# LoRa Concurrent Transmission Simulation

[English](README.md) | [中文](README_zh.md)

## Project Description

This project simulates concurrent LoRa packet transmissions to analyze interference patterns and network performance.

## Features
- Simulates multiple LoRa nodes transmitting simultaneously
- Simulates channel scheduling and allocation 
- Generates transmission statistics and performance metrics

## Installation
```bash
git clone git@github.com:Cresc3ntRose/LoRaSim-for-Concurrent-Transmission.git
cd lorasim_concurrent_transmission
cargo build --release
```

## Usage
1. Generate Random Packets
   ```bash
   cargo run -- --random
   ```
   This command generates random LoRa packets and saves them to the data directory.

2. Run Simulation with Existing Packets
   ```bash
   cargo run
   ```
   This command reads existing packets from the data directory and performs the simulation.

## Project Structure
```bash
src/
├── bin/
│   └── main.rs              # Entry point of the program
├── models/                  # Model definitions
│   ├── mod.rs               # Model module declaration
│   ├── packet.rs            # LoRa packet definition
│   └── gateway.rs           # Gateway definition
└── data_access/             # Data access layer
   ├── mod.rs               # Data access module declaration  
   ├── generate_packets.rs  # Generate random packets
   ├── get_packets.rs       # Read packets
   └── setup_logger.rs      # Logger configuration
```
```

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.