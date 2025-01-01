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

use crate::models::packet::Packet;
use crate::data_access::generate_packets::*;
use crate::data_access::get_packets::*;
use crate::data_access::setup_logger::*;

use log::{info, error, warn};
use chrono::{Local, Duration};
use rand::Rng;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Status of packet distribution
#[derive(Debug)]
pub enum DistributeStatus {
    Success(usize),          // Successfully allocated to a specific channel
    AllChannelsFull,         // All channels are full
    EmptyQueue,              // Pending queue is empty
    Timeout,                 // Packet timeout
}

#[derive(Debug, Clone)]
pub struct Gateway {
    pub id: u32,                                            // Gateway ID
    pub pending_queue: Arc<Mutex<VecDeque<Packet>>>,        // Pending queue for packets waiting to be processed
    pub channel_queues: Arc<Mutex<[VecDeque<Packet>; 8]>>,  // Channel queues for packets being processed, 8 channels in total
    pub pending_queue_capacity: usize,                      // Capacity of the pending queue
    pub channel_queue_capacity: usize,                      // Capacity of each channel queue
    pub should_exit: Arc<std::sync::atomic::AtomicBool>,    // Flag to indicate whether the gateway should exit
    pub time_threshold: Duration,                           // Time threshold for packet timeout
}

impl Gateway {
    /// Create a new gateway
    pub fn new(id: u32) -> Self {
        Gateway {
            id,
            pending_queue: Arc::new(Mutex::new(VecDeque::new())),
            channel_queues: Arc::new(Mutex::new([
                                VecDeque::new(),
                                VecDeque::new(),
                                VecDeque::new(),
                                VecDeque::new(),
                                VecDeque::new(),
                                VecDeque::new(),
                                VecDeque::new(),
                                VecDeque::new(),
                            ])),
            pending_queue_capacity: 1000,
            channel_queue_capacity: 10,
            should_exit: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            time_threshold: Duration::seconds(10),
        }
    }

    /// Check if the pending queue is empty
    pub fn is_pending_queue_empty(&self) -> bool {
        let queue = self.pending_queue.lock().unwrap();
        queue.len() <= 0
    }

    // #[warn(dead_code)]
    // /// Check if the pending queue is full
    // pub fn is_pending_queue_full(&self) -> bool {
    //     let queue = self.pending_queue.lock().unwrap();
    //     queue.len() >= self.pending_queue_capacity
    // }

    /// Check if a specific channel queue is empty
    pub fn is_channel_queue_empty(&self, i: usize) -> bool {
        let queue = self.channel_queues.lock().unwrap();
        queue[i].len() <= 0
    }

    /// Check if a specific channel queue is full
    pub fn is_channel_queue_full(&self, i: usize) -> bool {
        let queue = self.channel_queues.lock().unwrap();
        queue[i].len() >= self.channel_queue_capacity
    }

    /// Check if the packet has timed out
    pub fn is_timeout(&self, packet: &Packet) -> bool {
        Local::now().signed_duration_since(packet.arrival_time) > self.time_threshold
    }

    /// Resend a packet by removing it from the current queue, updating its arrival time, and re-adding it to the pending queue
    pub fn resend_packet(&self) {
        let mut pending_queue = self.pending_queue.lock().unwrap();
        let mut packet = pending_queue.pop_front().unwrap();
        warn!("\u{1F62D}: Packet {} is timeout, resent", packet.packet_id);
        // if self.is_pending_queue_full() {
        //     panic!("Pending queue is full, cannot resend packet");
        // }
        packet.arrival_time = Local::now();
        pending_queue.push_back(packet);
    }

    // /// Distribute one packet to the channel queues in a round-robin manner
    // pub fn distribute_one_packet(&self) -> DistributeStatus {
    //     {
    //         if self.is_pending_queue_empty() {
    //             return DistributeStatus::EmptyQueue;
    //         }
    //     }
    
    //     {
    //         let mut pending_queue = self.pending_queue.lock().unwrap();
    
    //         let packet = match pending_queue.front() {
    //             Some(p) => p,
    //             None => {
    //                 error!("Unexpected empty pending queue");
    //                 return DistributeStatus::EmptyQueue;
    //             }
    //         };
    
    //         if self.is_timeout(packet) {
    //             return DistributeStatus::Timeout;
    //         }
    
    //         for i in 0..8 {
    //             if !self.is_channel_queue_full(i) {
    //                 info!("Packet {} is allocated to channel {}", packet.packet_id, i);
    //                 let packet = pending_queue.pop_front().unwrap();
    //                 let mut channel_queues = self.channel_queues.lock().unwrap();
    //                 channel_queues[i].push_back(packet);
    //                 return DistributeStatus::Success(i);
    //             }
    //         }
    //     }
    
    //     info!("\u{1F613}: All channels are full");
    //     DistributeStatus::AllChannelsFull
    // }

    // /// Distribute one packet to the channel queues in a random manner
    // pub fn distribute_one_packet(&self) -> DistributeStatus {
    //     {
    //         if self.is_pending_queue_empty() {
    //             return DistributeStatus::EmptyQueue;
    //         }
    //     }
    
    //     {
    //         let mut pending_queue = self.pending_queue.lock().unwrap();
    
    //         let packet = match pending_queue.front() {
    //             Some(p) => p,
    //             None => {
    //                 error!("Unexpected empty pending queue");
    //                 return DistributeStatus::EmptyQueue;
    //             }
    //         };
    
    //         if self.is_timeout(packet) {
    //             return DistributeStatus::Timeout;
    //         }
    
    //         let mut rng = rand::thread_rng();
    //         let mut allocated = false;
    //         let mut i = 0;
    //         while i < 8 {
    //             let j = rng.gen_range(0..8);
    //             if !self.is_channel_queue_full(j) {
    //                 info!("Packet {} is allocated to channel {}", packet.packet_id, j);
    //                 let packet = pending_queue.pop_front().unwrap();
    //                 let mut channel_queues = self.channel_queues.lock().unwrap();
    //                 channel_queues[j].push_back(packet);
    //                 allocated = true;
    //                 break;
    //             }
    //             i += 1;
    //         }

    //         if allocated {
    //             return DistributeStatus::Success(i);
    //         }
    //     }
    
    //     info!("\u{1F613}: All channels are full");
    //     DistributeStatus::AllChannelsFull
    // }

    // /// Distribute one packet to the channel queues based on the load
    // pub fn distribute_one_packet(&self) -> DistributeStatus {
    //     {
    //         if self.is_pending_queue_empty() {
    //             return DistributeStatus::EmptyQueue;
    //         }
    //     }
    
    //     {
    //         let mut pending_queue = self.pending_queue.lock().unwrap();
    
    //         let packet = match pending_queue.front() {
    //             Some(p) => p,
    //             None => {
    //                 error!("Unexpected empty pending queue");
    //                 return DistributeStatus::EmptyQueue;
    //             }
    //         };
    
    //         if self.is_timeout(packet) {
    //             return DistributeStatus::Timeout;
    //         }

    //         if !(0..8).any(|i| !self.is_channel_queue_full(i)) {
    //             info!("\u{1F613}: All channels are full");
    //             return DistributeStatus::AllChannelsFull;
    //         }

    //         let channel_index = (0..8)
    //             .filter(|i| !self.is_channel_queue_full(*i))
    //             .min_by_key(|i| {
    //                 let channel_queues = self.channel_queues.lock().unwrap();
    //                 channel_queues[*i].len()
    //             })
    //             .unwrap();

    //         if !self.is_channel_queue_full(channel_index) {
    //             info!("Packet {} is allocated to channel {}", packet.packet_id, channel_index);
    //             let packet = pending_queue.pop_front().unwrap();
    //             let mut channel_queues = self.channel_queues.lock().unwrap();
    //             channel_queues[channel_index].push_back(packet);
    //             return DistributeStatus::Success(channel_index);
    //         }
    //     }
    
    //     info!("\u{1F613}: All channels are full");
    //     DistributeStatus::AllChannelsFull
    // }

    // /// Distribute one packet to the channel queues based on the time load
    // pub fn distribute_one_packet(&self) -> DistributeStatus {
    //     {
    //         if self.is_pending_queue_empty() {
    //             return DistributeStatus::EmptyQueue;
    //         }
    //     }
    
    //     {
    //         let mut pending_queue = self.pending_queue.lock().unwrap();
    
    //         let packet = match pending_queue.front() {
    //             Some(p) => p,
    //             None => {
    //                 error!("Unexpected empty pending queue");
    //                 return DistributeStatus::EmptyQueue;
    //             }
    //         };
    
    //         if self.is_timeout(packet) {
    //             return DistributeStatus::Timeout;
    //         }

    //         // Create a tuple containing the channel index and the total time of the channel queue
    //         let mut channel_time: Vec<(usize, Duration)> = (0..8)
    //             .map(|i| {
    //                 let channel_queues = self.channel_queues.lock().unwrap();
    //                 let total_time = channel_queues[i].iter().map(|p| p.processing_time).sum();
    //                 (i, total_time)
    //             })
    //             .collect();

    //         // Sort by total_time in ascending order
    //         channel_time.sort_by_key(|(_, total_time)| *total_time);

    //         // Iterate over the channel_time's usize
    //         for (i, _) in channel_time {
    //             if !self.is_channel_queue_full(i) {
    //                 info!("Packet {} is allocated to channel {}", packet.packet_id, i);
    //                 let packet = pending_queue.pop_front().unwrap();
    //                 let mut channel_queues = self.channel_queues.lock().unwrap();
    //                 channel_queues[i].push_back(packet);
    //                 return DistributeStatus::Success(i);
    //             }
    //         }
    //     }
    
    //     info!("\u{1F613}: All channels are full");
    //     DistributeStatus::AllChannelsFull
    // }

    /// Distribute one packet to the channel queues based on the time load with pending queue optimization
    pub fn distribute_one_packet(&self) -> DistributeStatus {
        {
            if self.is_pending_queue_empty() {
                return DistributeStatus::EmptyQueue;
            }
        }
    
        {
            let pending_queue = self.pending_queue.lock().unwrap();
    
            let packet = match pending_queue.front() {
                Some(p) => p,
                None => {
                    error!("Unexpected empty pending queue");
                    return DistributeStatus::EmptyQueue;
                }
            };
    
            if self.is_timeout(packet) {
                return DistributeStatus::Timeout;
            }
        }

        {
            // For each packet in the pending_queue, sort them by processing_time, 
            // with shorter processing_time being processed first
            let mut pending_queue = self.pending_queue.lock().unwrap();
            pending_queue.make_contiguous().sort_by_key(|p| p.processing_time);

            // Create a tuple containing the channel index and the total time of the channel queue
            let mut channel_time: Vec<(usize, Duration)> = (0..8)
                .map(|i| {
                    let channel_queues = self.channel_queues.lock().unwrap();
                    let total_time = channel_queues[i].iter().map(|p| p.processing_time).sum();
                    (i, total_time)
                })
                .collect();

            // Sort by total_time in ascending order
            channel_time.sort_by_key(|(_, total_time)| *total_time);

            // Iterate over the channel_time's usize
            for (i, _) in channel_time {
                if !self.is_channel_queue_full(i) {
                    let packet = pending_queue.pop_front().unwrap();
                    info!("Packet {} is allocated to channel {}", packet.packet_id, i);
                    let mut channel_queues = self.channel_queues.lock().unwrap();
                    channel_queues[i].push_back(packet);
                    return DistributeStatus::Success(i);
                }
            }
        }
    
        info!("\u{1F613}: All channels are full");
        DistributeStatus::AllChannelsFull
    }

    /// Initialize the gateway
    pub fn init(&self) {
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
            let mut pending_queue = self.pending_queue.lock().unwrap();
            pending_queue.clear();
            pending_queue.extend(packet_queue);
        }
    }

    pub fn simulation(&self) {
        self.init();
        let gateway = Arc::new(self.clone());

        let start_time = Instant::now();

        let mut threads = vec![];
        for i in 0..8 {
            let gateway = Arc::clone(&gateway);
            let handle = std::thread::spawn(move || {
                while !gateway.should_exit.load(std::sync::atomic::Ordering::Relaxed) {
                    // 1. Check if the channel queue is empty
                    if gateway.is_channel_queue_empty(i) {
                        warn!("Channel {} is empty, waiting...", i);
                        std::thread::sleep(std::time::Duration::from_millis(1));
                        continue;
                    }
                
                    // 2. Process the packet
                    let packet = {
                        let mut channel_queues = gateway.channel_queues.lock().unwrap();
                        let packet = channel_queues[i].pop_front().unwrap();
                        packet
                    };
                
                    info!("\u{1F600}: Packet {} is processing in channel {}", packet.packet_id, i);
                    
                    // 3. Add timeout detection
                    let start = std::time::Instant::now();
                    let processing_time_std = std::time::Duration::from_millis(
                        packet.processing_time.num_milliseconds() as u64
                    );
                    
                    std::thread::sleep(processing_time_std);
                    
                    let elapsed = start.elapsed();
                    info!("\u{1F60A}: Packet {} processed in channel {} (took {:?})", 
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
            match gateway.distribute_one_packet() {
                DistributeStatus::Success(_) => {
                    std::thread::sleep(std::time::Duration::from_micros(100));
                }
                DistributeStatus::AllChannelsFull => {
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                DistributeStatus::Timeout => {
                    gateway.resend_packet();
                    std::thread::sleep(std::time::Duration::from_micros(500));
                }
                DistributeStatus::EmptyQueue => {
                    gateway.should_exit.store(true, std::sync::atomic::Ordering::Relaxed);
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
}