//! hyperlane-log
//!
//! A Rust logging library that supports both asynchronous and synchronous logging.
//! It provides multiple log levels, such as error, info, and debug.
//! Users can define custom log handling methods and configure log file paths.
//! The library supports log rotation, automatically creating a new log file
//! when the current file reaches the specified size limit.
//! It allows flexible logging configurations, making it suitable for
//! both high-performance asynchronous applications and
//! traditional synchronous logging scenarios. The asynchronous mode utilizes
//! Tokio's async channels for efficient log buffering,
//! while the synchronous mode writes logs directly to the file system.

#[cfg(test)]
pub(crate) mod cfg;
pub(crate) mod log;

pub use log::*;

pub(crate) use file_operation::*;
pub(crate) use hyperlane_time::*;
pub(crate) use std::fs::read_dir;
