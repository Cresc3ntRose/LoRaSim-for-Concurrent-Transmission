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

#[path = "../models/mod.rs"]
mod models;

#[path = "../data_access/mod.rs"]
mod data_access;

use crate::models::packet::Packet;
use crate::data_access::generate_packets::*;
use crate::data_access::get_packets::*;

use log::{info, error, warn, debug};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::collections::VecDeque;
use std::sync::Mutex;
use lazy_static::lazy_static;
use chrono::{Local, Duration};
use std::time::Instant;

/// Pending queue for packets waiting to be processed
pub static PENDING_QUEUE: Mutex<VecDeque<Packet>> = Mutex::new(VecDeque::new());

lazy_static! {
    /// Channel queues for packets being processed, 8 channels in total
    pub static ref CHANNEL_QUEUES: Mutex<[VecDeque<Packet>; 8]> = Mutex::new([
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
    ]);
}

/// Capacity of the pending queue
pub const PENDING_QUEUE_CAPACITY: usize = 1000;

/// Capacity of each channel queue
pub const CHANNEL_QUEUE_CAPACITY: usize = 10;

/// Status of each channel, whether it is processing a packet
pub static CHANNEL: Mutex<[bool; 8]> = Mutex::new([false; 8]);

/// Flag to indicate whether the process should exit
pub static SHOULD_EXIT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Time threshold for packet timeout
pub const TIME_THRESHOLD: Duration = Duration::seconds(10);

/// Check if the pending queue is empty
pub fn is_pending_queue_empty() -> bool {
    let queue = PENDING_QUEUE.lock().unwrap();
    queue.len() == 0
}

/// Check if the pending queue is full
pub fn is_pending_queue_full() -> bool {
    let queue = PENDING_QUEUE.lock().unwrap();
    queue.len() >= PENDING_QUEUE_CAPACITY
}

/// Check if a specific channel queue is empty
pub fn is_channel_queue_empty(i: usize) -> bool {
    let queue = CHANNEL_QUEUES.lock().unwrap();
    queue[i].len() == 0
}

/// Check if a specific channel queue is full
pub fn is_channel_queue_full(i: usize) -> bool {
    let queue = CHANNEL_QUEUES.lock().unwrap();
    queue[i].len() >= CHANNEL_QUEUE_CAPACITY
}

/// Check if the packet has timed out
pub fn is_timeout(packet: &Packet) -> bool {
    Local::now().signed_duration_since(packet.arrival_time) > TIME_THRESHOLD
}

/// Resend a packet by removing it from the current queue, updating its arrival time, and re-adding it to the pending queue
pub fn resend_packet() {
    let mut pending_queue = PENDING_QUEUE.lock().unwrap();
    let mut packet = pending_queue.pop_front().unwrap();
    warn!("Packet {} is timeout, resent", packet.packet_id);
    packet.arrival_time = Local::now();
    pending_queue.push_back(packet);
}

/// Status of packet distribution
#[derive(Debug)]
pub enum DistributeStatus {
    Success(usize),          // Successfully allocated to a specific channel
    AllChannelsFull,         // All channels are full
    EmptyQueue,              // Pending queue is empty
    Timeout,                 // Packet timeout
}

/// Distribute one packet to the channel queues in a round-robin manner
pub fn distribute_one_packet() -> DistributeStatus {
    {
        let pending_queue = PENDING_QUEUE.lock().unwrap();
        if pending_queue.is_empty() {
            return DistributeStatus::EmptyQueue;
        }
    }

    {
        let mut pending_queue = PENDING_QUEUE.lock().unwrap();

        let packet = match pending_queue.front() {
            Some(p) => p,
            None => {
                error!("Unexpected empty pending queue");
                return DistributeStatus::EmptyQueue;
            }
        };

        if is_timeout(packet) {
            return DistributeStatus::Timeout;
        }

        for i in 0..8 {
            if !is_channel_queue_full(i) {
                info!("Packet {} is allocated to channel {}", packet.packet_id, i);
                let packet = pending_queue.pop_front().unwrap();
                let mut channel_queues = CHANNEL_QUEUES.lock().unwrap();
                channel_queues[i].push_back(packet);
                return DistributeStatus::Success(i);
            }
        }
    }

    info!("All channels are full");
    DistributeStatus::AllChannelsFull
}

/// Setup the logger for the application
fn setup_logger() -> Result<(), fern::InitError> {
    let log_file_path = "logs/simulation.log";
    
    if Path::new(log_file_path).exists() {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(log_file_path)
            .unwrap();
        file.write_all(b"").unwrap();
    }
    
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] [{}]\n\t{}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file(log_file_path)?)
        .apply()?;
    Ok(())
}

/// Initialize the application
pub fn init() {
    setup_logger().unwrap();

    let args: Vec<String> = std::env::args().collect();
    let packet_queue = if args.contains(&String::from("--random")) {
        match generate_random_packet() {
            Ok(queue) => {
                info!("Generated random packets: {}", queue.len());
                queue
            },
            Err(e) => {
                error!("Failed to generate packets: {}", e);
                panic!("Failed to generate packets: {}", e);
            }
        }
    } else {
        match load_data() {
            Ok(queue) => {
                info!("Loaded packets from file: {}", queue.len());
                queue
            },
            Err(e) => {
                error!("Failed to load packets: {}", e);
                panic!("Failed to load packets: {}", e);
            }
        }
    };

    {
        let mut pending_queue = PENDING_QUEUE.lock().unwrap();
        pending_queue.clear();
        pending_queue.extend(packet_queue);
    }
}

fn main() {
    init();

    let start_time = Instant::now();

    let mut threads = vec![];
    for i in 0..8 {
        let handle = std::thread::spawn(move || {
            while !SHOULD_EXIT.load(std::sync::atomic::Ordering::Relaxed) {
                // 1. Check if the channel queue is empty
                if is_channel_queue_empty(i) {
                    debug!("Channel {} is empty, waiting...", i);
                    std::thread::sleep(std::time::Duration::from_millis(1));
                    continue;
                }
            
                // 2. Process the packet
                let packet = {
                    let mut channel_queues = CHANNEL_QUEUES.lock().unwrap();
                    let packet = channel_queues[i].pop_front().unwrap();
                    packet
                };
            
                info!("Packet {} is processing in channel {}", packet.packet_id, i);
                
                // 3. Add timeout detection
                let start = std::time::Instant::now();
                let processing_time_std = std::time::Duration::from_millis(
                    packet.processing_time.num_milliseconds() as u64
                );
                
                std::thread::sleep(processing_time_std);
                
                let elapsed = start.elapsed();
                info!("Packet {} processed in channel {} (took {:?})", 
                    packet.packet_id, i, elapsed);
                
                if elapsed > processing_time_std * 2 {
                    warn!("Packet {} processing took longer than expected in channel {}", 
                        packet.packet_id, i);
                }
            }
        });
        threads.push(handle);
    }

    loop {
        match distribute_one_packet() {
            DistributeStatus::Success(_) => {
                std::thread::sleep(std::time::Duration::from_micros(100));
            }
            DistributeStatus::AllChannelsFull => {
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            DistributeStatus::Timeout => {
                resend_packet();
                std::thread::sleep(std::time::Duration::from_micros(500));
            }
            DistributeStatus::EmptyQueue => {
                SHOULD_EXIT.store(true, std::sync::atomic::Ordering::Relaxed);
                break;
            }
        }
    }

    for handle in threads {
        handle.join().unwrap();
    }

    info!("All packets are processed, waiting for channels to finish");

    let elapsed = start_time.elapsed();
    println!("Total processing time: {:?}", elapsed);
}
