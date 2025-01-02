/*
 * Copyright (C) 2025 [Yuxuan Huang - NUAA]
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::models::packet::Packet;
use log::{info, error};
use rand::Rng;
use chrono::Duration;
use serde_json::json;

use std::error::Error;
use std::fs::File;
use std::collections::VecDeque;

/// Setup data for the simulation
fn setup_data(packets: &VecDeque<Packet>) -> Result<(), Box<dyn Error>> {
    let data_file_path = "data/packet.json";
    let file = File::create(data_file_path)?;
    
    let json_packets: Vec<_> = packets.iter().map(|packet| {
        json!({
            "packet_id": packet.packet_id,
            "arrival_time": packet.arrival_time,
            "processing_time": packet.processing_time,
        })
    }).collect();
    
    serde_json::to_writer_pretty(file, &json_packets)?;
    Ok(())
}

/// Generate random packets for the simulation
pub fn generate_random_packet() -> Result<VecDeque<Packet>, Box<dyn Error>> {

    let mut rng = rand::thread_rng();
    let mut packets: Vec<Packet> = Vec::with_capacity(1000);

    info!("Start generating random packets");

    for packet_id in 0..1000 {
        // Generate a random number between 50 and 150 for processing time (milliseconds)
        let processing_time_ms = rng.gen_range(50..=150);
        let processing_time = Duration::milliseconds(processing_time_ms);

        // Use Packet::new to create a packet, automatically setting the arrival time
        let packet = Packet::new(packet_id, processing_time);

        packets.push(packet);
    }

    // Sort by arrival time
    packets.sort_by(|a, b| a.arrival_time.cmp(&b.arrival_time));

    // Convert to VecDeque
    let packet_queue: VecDeque<Packet> = VecDeque::from(packets);

    // Check the length of the packet queue
    if packet_queue.len() > 1000 {
        error!("Packet queue size exceeded 1000, current size: {}", packet_queue.len());
        panic!("Packet queue size exceeded 1000");
    }

    setup_data(&packet_queue)?;

    info!("Successfully generated {} packets\n", packet_queue.len());

    Ok(packet_queue)
}