use crate::Log;
use recoverable_spawn::{sync::*, JoinHandle};
use std::{sync::Arc, thread::sleep, time::Duration};

#[inline]
pub fn log_run(log: &Log) -> JoinHandle<()> {
    let arc_log: Arc<Log> = Arc::new(log.clone());
    let log_thread: JoinHandle<()> = recoverable_spawn(move || {
        let arc_error_log_clone: Arc<Log> = Arc::clone(&arc_log);
        let arc_info_log_clone: Arc<Log> = Arc::clone(&arc_log);
        let arc_debug_log_clone: Arc<Log> = Arc::clone(&arc_log);
        let error_log_thread: JoinHandle<()> = recoverable_spawn(move || {
            let interval_millis: u64 = *arc_error_log_clone.get_interval_millis() as u64;
            loop {
                arc_error_log_clone.write_error();
                sleep(Duration::from_millis(interval_millis));
            }
        });
        let info_log_thread: JoinHandle<()> = recoverable_spawn(move || {
            let interval_millis: u64 = *arc_info_log_clone.get_interval_millis() as u64;
            loop {
                arc_info_log_clone.write_info();
                sleep(Duration::from_millis(interval_millis));
            }
        });
        let debug_log_thread: JoinHandle<()> = recoverable_spawn(move || {
            let interval_millis: u64 = *arc_debug_log_clone.get_interval_millis() as u64;
            loop {
                arc_debug_log_clone.write_debug();
                sleep(Duration::from_millis(interval_millis));
            }
        });
        let _ = error_log_thread.join();
        let _ = info_log_thread.join();
        let _ = debug_log_thread.join();
    });
    log_thread
}
