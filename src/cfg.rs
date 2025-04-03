#[tokio::test]
async fn test() {
    use crate::*;
    use std::thread;
    use std::time::Duration;
    let log: Log = Log::new("./logs", 1_024_000);
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
    log.async_error("async error data", |error| {
        let write_data: String = format!("User error func =>  {:?}\n", error);
        write_data
    })
    .await;
    log.async_info("async info data", |info| {
        let write_data: String = format!("User info func =>  {:?}\n", info);
        write_data
    })
    .await;
    log.async_debug("async debug data", |debug| {
        let write_data: String = format!("User debug func =>  {:#?}\n", debug);
        write_data
    })
    .await;
    thread::sleep(Duration::new(6, 0));
}

#[tokio::test]
async fn test_more_log_first() {
    use crate::*;
    use std::thread;
    use std::time::Duration;
    let log: Log = Log::new("./logs", DISABLE_LOG_FILE_SIZE);
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
    log.async_error("async error data => ", |error| {
        let write_data: String = format!("User error func =>  {:?}\n", error);
        write_data
    })
    .await;
    log.async_info("async info data => ", |info| {
        let write_data: String = format!("User info func =>  {:?}\n", info);
        write_data
    })
    .await;
    log.async_debug("async debug data => ", |debug| {
        let write_data: String = format!("User debug func =>  {:#?}\n", debug);
        write_data
    })
    .await;
    thread::sleep(Duration::new(6, 0));
}

#[tokio::test]
async fn test_more_log_second() {
    use crate::*;
    use std::thread;
    use std::time::Duration;
    for _ in 0..10 {
        let log: Log = Log::new("./logs", 512_000);
        loop {
            log.error("error data!", |error| {
                let write_data: String = format!("User error func =>  {:?}\n", error);
                write_data
            });
            log.async_error("async error data!", |error| {
                let write_data: String = format!("User error func =>  {:?}\n", error);
                write_data
            })
            .await;
        }
    }
    thread::sleep(Duration::new(6, 0));
}
