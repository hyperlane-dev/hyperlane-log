#[test]
fn test() {
    use crate::*;
    use std::thread;
    use std::time::Duration;
    let log: Log = Log::new("./logs", 1_024_000);
    let _log_thread: JoinHandle<()> = log_run(&log);
    log.log_error("error data => ", |error| {
        let write_data: String = format!("User error func =>  {:?}\n", error);
        write_data
    });
    log.log_info("info data => ", |info| {
        let write_data: String = format!("User info func =>  {:?}\n", info);
        write_data
    });
    log.log_debug("debug data => ", |debug| {
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
    let log: Log = Log::new("./logs", 0);
    let _log_thread: JoinHandle<()> = log_run(&log);
    log.log_error("error data => ", |error| {
        let write_data: String = format!("User error func =>  {:?}\n", error);
        write_data
    });
    log.log_info("info data => ", |info| {
        let write_data: String = format!("User info func =>  {:?}\n", info);
        write_data
    });
    log.log_debug("debug data => ", |debug| {
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
    let log: Log = Log::new("./logs", 0);
    let _log_thread: JoinHandle<()> = log_run(&log);
    log.log_error("error data => ", |error| {
        let write_data: String = format!("User error func =>  {:?}\n", error);
        write_data
    });
    log.log_info("info data => ", |info| {
        let write_data: String = format!("User info func =>  {:?}\n", info);
        write_data
    });
    log.log_debug("debug data => ", |debug| {
        let write_data: String = format!("User debug func =>  {:#?}\n", debug);
        write_data
    });
    thread::sleep(Duration::new(6, 0));
}

#[test]
fn read_log() {
    use crate::*;
    use std::error::Error;
    let data: Result<DataString, Box<dyn Error>> = read_from_file("./.gitignore");
    println!("{:?}", data);
    let data: Result<Vec<u8>, Box<dyn Error>> = read_from_file("./.error");
    println!("{:?}", data);
}
