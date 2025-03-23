#[test]
fn test() {
    use crate::*;
    use std::thread;
    use std::time::Duration;
    let log: Log = Log::new("./logs", 1_024_000, 360);
    let _log_thread: JoinHandle<()> = log_run(&log);
    log.error("error data", |error| {
        let write_data: String = format!("User error func =>  {:?}\n", error);
        write_data
    });
    log.info("info data", |info| {
        let write_data: String = format!("User info func =>  {:?}\n", info);
        write_data
    });
    log.debug("debug data", |debug| {
        let write_data: String = format!("User debug func =>  {:#?}\n", debug);
        write_data
    });
    thread::sleep(Duration::new(6, 0));
}

#[test]
fn test_more_log_first() {
    use crate::*;
    use std::thread;
    use std::time::Duration;
    let log: Log = Log::new("./logs", 0, 360);
    let _log_thread: JoinHandle<()> = log_run(&log);
    log.error("error data => ", |error| {
        let write_data: String = format!("User error func =>  {:?}\n", error);
        write_data
    });
    log.info("info data => ", |info| {
        let write_data: String = format!("User info func =>  {:?}\n", info);
        write_data
    });
    log.debug("debug data => ", |debug| {
        let write_data: String = format!("User debug func =>  {:#?}\n", debug);
        write_data
    });
    thread::sleep(Duration::new(6, 0));
}

#[test]
fn test_more_log_second() {
    use crate::*;
    use std::thread;
    use std::time::Duration;
    thread::sleep(Duration::new(4, 0));
    let log: Log = Log::new("./logs", 360, 0);
    let _log_thread: JoinHandle<()> = log_run(&log);
    let times: i32 = 1000;
    for _ in 0..times {
        log.error("error data!", |error| {
            let write_data: String = format!("User error func =>  {:?}\n", error);
            write_data
        });
    }
    for _ in 0..times {
        log.info("info data!", |info| {
            let write_data: String = format!("User info func =>  {:?}\n", info);
            write_data
        });
    }
    for _ in 0..times {
        log.debug("debug data!", |debug| {
            let write_data: String = format!("User debug func =>  {:#?}\n", debug);
            write_data
        });
    }
    thread::sleep(Duration::new(4, 0));
}
