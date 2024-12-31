/*
 * Copyright (C) 2024 [Yuxuan Huang - NUAA]
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

use chrono::{DateTime, Local, Duration};
use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Packet {
    pub packet_id: u32,                // Packet ID
    pub arrival_time: DateTime<Local>, // Arrival time
    pub processing_time: Duration,     // Processing time required
}

impl Packet {
    pub fn new(packet_id: u32, processing_time: Duration) -> Self {
        let local_now = Local::now();
        info!("Packet created: {:?},\n\tArrival time: {:?},\n\tProcessing time: {:?} ms", packet_id, local_now, processing_time.num_milliseconds());
        Packet {
            packet_id,
            arrival_time: local_now,
            processing_time,
        }
    }
}