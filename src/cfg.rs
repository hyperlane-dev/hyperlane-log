#[test]
fn test() {
    use crate::*;
    let log: Log = Log::new("./logs", 1_024_000);
    log_run(&log);
    log.log_error("error data => ", |error| {
        let result = format!("User func =>  {:?}", error);
        result
    });
    log.log_info("info data => ", |error| {
        let result = format!("User func =>  {:?}", error);
        result
    });
    log.log_debug("debug data => ", |error| {
        let result = format!("User func =>  {:#?}", error);
        result
    });
    loop {}
}
