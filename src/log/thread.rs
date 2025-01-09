use crate::{Log, LogArcLock};
use lazy_static::lazy_static;
use recoverable_spawn::*;
use std::sync::{Arc, RwLock};

lazy_static! {
    static ref LOG: LogArcLock = Arc::new(RwLock::new(Log::default()));
}

#[inline]
pub fn log_run(log: &Log) -> JoinHandle<()> {
    let log_clone: Log = log.clone();
    {
        if let Ok(mut log) = LOG.write() {
            *log = log_clone;
        }
    }
    let log_thread: JoinHandle<()> = recoverable_spawn(move || {
        let error_log_thread: JoinHandle<()> = recoverable_spawn(move || loop {
            if let Ok(log) = LOG.read() {
                log.write_error();
            }
        });
        let info_log_thread: JoinHandle<()> = recoverable_spawn(move || loop {
            if let Ok(log) = LOG.read() {
                log.write_info();
            }
        });
        let debug_log_thread: JoinHandle<()> = recoverable_spawn(move || loop {
            if let Ok(log) = LOG.read() {
                log.write_debug();
            }
        });
        let _ = error_log_thread.join();
        let _ = info_log_thread.join();
        let _ = debug_log_thread.join();
    });
    log_thread
}
