#[cfg(test)]
#[tokio::test]
async fn test() {
    use crate::*;
    let log: FileLogger = FileLogger::new("./logs", 1_024_000);
    let trace_str: String = String::from("custom trace message");
    log.trace(trace_str, |trace| {
        let write_data: String = format!("User trace func => {trace:#?}\n");
        write_data
    });
    let debug_str: String = String::from("custom debug message");
    log.debug(debug_str, |debug| {
        let write_data: String = format!("User debug func => {debug:#?}\n");
        write_data
    });
    let info_str: String = String::from("custom info message");
    log.info(info_str, |info| {
        let write_data: String = format!("User info func => {info:?}\n");
        write_data
    });
    let warn_str: String = String::from("custom warn message");
    log.warn(warn_str, |warn| {
        let write_data: String = format!("User warn func => {warn:#?}\n");
        write_data
    });
    let error_str: String = String::from("custom error message");
    log.error(error_str, |error| {
        let write_data: String = format!("User error func => {error:?}\n");
        write_data
    });
    let async_trace_str: String = String::from("custom async trace message");
    log.async_trace(async_trace_str, |trace| {
        let write_data: String = format!("User trace func => {trace:#?}\n");
        write_data
    })
    .await;
    let async_debug_str: String = String::from("custom async debug message");
    log.async_debug(async_debug_str, |debug| {
        let write_data: String = format!("User debug func => {debug:#?}\n");
        write_data
    })
    .await;
    let async_info_str: String = String::from("custom async info message");
    log.async_info(async_info_str, |info| {
        let write_data: String = format!("User info func => {info:?}\n");
        write_data
    })
    .await;
    let async_warn_str: String = String::from("custom async warn message");
    log.async_warn(async_warn_str, |warn| {
        let write_data: String = format!("User warn func => {warn:#?}\n");
        write_data
    })
    .await;
    let async_error_str: String = String::from("custom async error message");
    log.async_error(async_error_str, |error| {
        let write_data: String = format!("User error func => {error:?}\n");
        write_data
    })
    .await;
}

#[cfg(test)]
#[tokio::test]
async fn test_more_log_first() {
    use crate::*;
    let log: FileLogger = FileLogger::new("./logs", DISABLE_LOG_FILE_SIZE);
    log.trace("trace data => ", |trace| {
        let write_data: String = format!("User trace func => {trace:#?}\n");
        write_data
    });
    log.debug("debug data => ", |debug| {
        let write_data: String = format!("User debug func => {debug:#?}\n");
        write_data
    });
    log.info("info data => ", |info| {
        let write_data: String = format!("User info func => {info:?}\n");
        write_data
    });
    log.warn("warn data => ", |warn| {
        let write_data: String = format!("User warn func => {warn:#?}\n");
        write_data
    });
    log.error("error data => ", |error| {
        let write_data: String = format!("User error func => {error:?}\n");
        write_data
    });
    log.async_trace("async trace data => ", |trace| {
        let write_data: String = format!("User trace func => {trace:#?}\n");
        write_data
    })
    .await;
    log.async_debug("async debug data => ", |debug| {
        let write_data: String = format!("User debug func => {debug:#?}\n");
        write_data
    })
    .await;
    log.async_info("async info data => ", |info| {
        let write_data: String = format!("User info func => {info:?}\n");
        write_data
    })
    .await;
    log.async_warn("async warn data => ", |warn| {
        let write_data: String = format!("User warn func => {warn:#?}\n");
        write_data
    })
    .await;
    log.async_error("async error data => ", |error| {
        let write_data: String = format!("User error func => {error:?}\n");
        write_data
    })
    .await;
}

#[cfg(test)]
#[tokio::test]
async fn test_more_log_second() {
    use crate::*;
    for _ in 0..10 {
        let log: FileLogger = FileLogger::new("./logs", 512_000);
        log.trace("trace data!\n", common_log);
        log.async_trace("async trace data!\n", common_log).await;
        log.debug("debug data!\n", common_log);
        log.async_debug("async debug data!\n", common_log).await;
        log.info("info data!\n", common_log);
        log.async_info("async info data!\n", common_log).await;
        log.warn("warn data!\n", common_log);
        log.async_warn("async warn data!\n", common_log).await;
        log.error("error data!\n", common_log);
        log.async_error("async error data!\n", common_log).await;
    }
}
