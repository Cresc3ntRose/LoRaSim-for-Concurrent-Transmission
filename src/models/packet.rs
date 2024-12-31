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