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

use crate::models::gateway::*;

fn main() {
    let gateway = Gateway::new(0);

    gateway.simulation();
}
