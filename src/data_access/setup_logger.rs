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

use once_cell::sync::OnceCell;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

static LOGGER: OnceCell<()> = OnceCell::new();

/// Setup the logger for the application
pub fn setup_logger() -> Result<(), fern::InitError> {
    LOGGER.get_or_try_init(|| {
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
    }).map(|_| ())
}