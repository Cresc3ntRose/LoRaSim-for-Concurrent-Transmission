use crate::models::packet::Packet;
use log::error;
use log::info;
use serde_json::from_reader;
use std::error::Error;
use std::collections::VecDeque;
use std::fs::File;
use std::path::Path;

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