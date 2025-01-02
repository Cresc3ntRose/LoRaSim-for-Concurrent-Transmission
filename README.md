# LoRa Concurrent Transmission Simulation

[English](README.md) | [中文](README_zh.md)

## Project Overview
This project aims to simulate concurrent LoRa packet transmissions to analyze interference patterns and network performance. By simulating simultaneous transmissions from multiple LoRa nodes, it evaluates the effectiveness of channel scheduling and allocation strategies, and generates detailed transmission statistics and performance metrics.

## Features
- Concurrent transmission simulation
- Channel scheduling and allocation
- Queue management
- Timeout handling and retransmission mechanism
- Multithreading

## Installation
```bash {.line-numbers}
git clone git@github.com:Cresc3ntRose/LoRaSim-for-Concurrent-Transmission.git
cd lorasim_concurrent_transmission
mkdir logs data
cd logs && touch simulation.log
cd ../data && touch packet.json && cd ..
cargo build --release
```

## Usage
1. Generate Random Packets
   ```bash
   cargo run -- --random
   ```
   This command generates random LoRa packets, saves them locally, and performs the simulation using these packets.

2. Run Simulation with Existing Packets
   ```bash
   cargo run
   ```
   This command reads existing packets from the data directory and performs the simulation.

## Project Structure
```bash
src/
├── bin/
│   └── siomulation.rs       # Entry point of the program
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

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.