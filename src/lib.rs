// Copyright 2019 The Packem Maintainers. All rights reserved.
//
// Use of this source code is governed by a MIT license that can
// be found in the LICENSE file.
//
// This crate is based on Sam Clements' simple_logger crate but is
// modified to support more suitable error loggings for Packem's LC.
// A proposal to provide this support for the RC as well exists but is
// limited to the fact that there are no 'proper' bindings from Rust to
// Node. However, this won't be the case if such bindings are in place.
//

extern crate colored;
extern crate log;

use colored::*;

use log::{Level, Log, Metadata, Record, SetLoggerError};

struct PackemLogger {
    level: Level,
}

impl Log for PackemLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_string = {
                match record.level() {
                    Level::Error => format!(" {:?} ", record.level().to_string()).red(),
                    Level::Warn => format!(" {:?} ", record.level().to_string()).yellow(),
                    Level::Info => format!(" {:?} ", record.level().to_string()).cyan(),
                    Level::Debug => format!(" {:?} ", record.level().to_string()).purple(),
                    Level::Trace => format!(" {:?} ", record.level().to_string()).normal(),
                }
            };
            println!("{:<5} {}", level_string, record.args());
        }
    }

    fn flush(&self) {}
}

/// Initializes the global logger with a PackemLogger instance with
/// `max_log_level` set to a specific log level.
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    let logger = PackemLogger { level };

    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());

    Ok(())
}

/// Initializes the global logger with a PackemLogger instance with
/// `max_log_level` set to `LogLevel::Trace`.
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Trace)
}
