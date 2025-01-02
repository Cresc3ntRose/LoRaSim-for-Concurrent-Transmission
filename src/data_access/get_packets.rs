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
use log::error;
use log::info;
use serde_json::from_reader;

use std::error::Error;
use std::collections::VecDeque;
use std::fs::File;
use std::path::Path;

/// Load data from file
pub fn load_data() -> Result<VecDeque<Packet>, Box<dyn Error>> {

    let data_file_path = "data/packet.json";
    info!("Start loading data from file: {}", data_file_path);

    if !Path::new(data_file_path).exists() {
        error!("Data file not found: {}", data_file_path);
        panic!("Data file not found: {}", data_file_path);
    }

    let file = File::open(data_file_path)?;
    let tmp: Vec<Packet> = from_reader(file)?;

    let packets: Vec<Packet> = tmp.iter().map(|packet| {
        Packet::new(packet.packet_id, packet.processing_time)
    }).collect();

    // Check the length of the packet queue
    if packets.len() > 1000 {
        error!("Packet queue size exceeded 1000, current size: {}", packets.len());
        panic!("Packet queue size exceeded 1000");
    }

    info!("Successfully loaded {} packets\n", packets.len());

    Ok(VecDeque::from(packets))
}