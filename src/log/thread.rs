use crate::Log;
use std::thread::{spawn, JoinHandle};

#[inline]
pub fn log_run(log: &Log) -> JoinHandle<()> {
    let log_clone: Log = log.clone();
    let log_thread: JoinHandle<()> = spawn(move || loop {
        log_clone.write_error();
        log_clone.write_info();
        log_clone.write_debug();
    });
    log_thread
}
