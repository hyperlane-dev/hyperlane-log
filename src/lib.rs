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

mod r#const;
mod r#fn;
mod r#impl;
mod r#struct;
#[cfg(test)]
mod test;
mod r#trait;

pub use {r#const::*, r#fn::*, r#struct::*, r#trait::*};

use std::fs::read_dir;

use {file_operation::*, hyperlane_time::*};
