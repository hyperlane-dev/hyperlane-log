#[test]
fn test() {
    use crate::*;
    let log: Log = Log::new("./logs", 1_024_000);
    log_run(&log);
    log.log_error("error data => ", |error| {
        let result: String = format!("User error func =>  {:?}\n", error);
        result
    });
    log.log_info("info data => ", |info| {
        let result: String = format!("User info func =>  {:?}\n", info);
        result
    });
    log.log_debug("debug data => ", |debug| {
        let result: String = format!("User debug func =>  {:#?}\n", debug);
        result
    });
    loop {}
}
