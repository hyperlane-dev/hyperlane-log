use crate::*;

pub fn log_run(log: &Log) -> JoinHandle<()> {
    let arc_log: ArcLog = Arc::new(log.clone());
    let log_thread: JoinHandle<()> = recoverable_spawn(move || {
        let arc_log_clone: ArcLog = Arc::clone(&arc_log);
        let interval_millis: u64 = *arc_log_clone.get_interval_millis() as u64;
        if arc_log_clone.id_disable() {
            return;
        }
        let _ = recoverable_spawn(move || {
            loop {
                arc_log_clone.write_error();
                arc_log_clone.write_info();
                arc_log_clone.write_debug();
                sleep(Duration::from_millis(interval_millis));
            }
        })
        .join();
    });
    log_thread
}
