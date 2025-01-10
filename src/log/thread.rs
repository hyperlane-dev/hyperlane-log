use crate::Log;
use recoverable_spawn::*;
use std::sync::Arc;

#[inline]
pub fn log_run(log: &Log) -> JoinHandle<()> {
    let arc_log: Arc<Log> = Arc::new(log.clone());
    let log_thread: JoinHandle<()> = recoverable_spawn(move || {
        let arc_error_log_clone: Arc<Log> = Arc::clone(&arc_log);
        let arc_info_log_clone: Arc<Log> = Arc::clone(&arc_log);
        let arc_debug_log_clone: Arc<Log> = Arc::clone(&arc_log);
        let error_log_thread: JoinHandle<()> = recoverable_spawn(move || loop {
            arc_error_log_clone.write_error();
        });
        let info_log_thread: JoinHandle<()> = recoverable_spawn(move || loop {
            arc_info_log_clone.write_info();
        });
        let debug_log_thread: JoinHandle<()> = recoverable_spawn(move || loop {
            arc_debug_log_clone.write_debug();
        });
        let _ = error_log_thread.join();
        let _ = info_log_thread.join();
        let _ = debug_log_thread.join();
    });
    log_thread
}
