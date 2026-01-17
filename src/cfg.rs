use crate::*;

#[cfg(test)]
#[tokio::test]
async fn test() {
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

#[cfg(test)]
#[tokio::test]
async fn test_set_log_level_dirs() {
    let mut log: FileLogger = FileLogger::new("./test_logs", 1_024_000);
    log.set_trace_dir("custom_trace")
        .set_debug_dir("custom_debug")
        .set_info_dir("custom_info")
        .set_warn_dir("custom_warn")
        .set_error_dir("custom_error");
    assert_eq!(log.get_trace_dir(), "custom_trace");
    assert_eq!(log.get_debug_dir(), "custom_debug");
    assert_eq!(log.get_info_dir(), "custom_info");
    assert_eq!(log.get_warn_dir(), "custom_warn");
    assert_eq!(log.get_error_dir(), "custom_error");
    log.trace("test trace message", common_log);
    log.debug("test debug message", common_log);
    log.info("test info message", common_log);
    log.warn("test warn message", common_log);
    log.error("test error message", common_log);
    log.async_trace("async test trace message", common_log)
        .await;
    log.async_debug("async test debug message", common_log)
        .await;
    log.async_info("async test info message", common_log).await;
    log.async_warn("async test warn message", common_log).await;
    log.async_error("async test error message", common_log)
        .await;
}

#[cfg(test)]
#[tokio::test]
async fn test_log_level_dir_constants() {
    let log: FileLogger = FileLogger::default();
    assert_eq!(log.get_trace_dir(), TRACE_DIR);
    assert_eq!(log.get_debug_dir(), DEBUG_DIR);
    assert_eq!(log.get_info_dir(), INFO_DIR);
    assert_eq!(log.get_warn_dir(), WARN_DIR);
    assert_eq!(log.get_error_dir(), ERROR_DIR);
}

#[cfg(test)]
#[tokio::test]
async fn test_log_level_dir_method_chaining() {
    let mut log: FileLogger = FileLogger::new("./logs", 512_000);
    let log_ref: &mut FileLogger = log
        .set_trace_dir("chain_trace")
        .set_debug_dir("chain_debug")
        .set_info_dir("chain_info")
        .set_warn_dir("chain_warn")
        .set_error_dir("chain_error");
    assert_eq!(log_ref.get_trace_dir(), "chain_trace");
    assert_eq!(log_ref.get_debug_dir(), "chain_debug");
    assert_eq!(log_ref.get_info_dir(), "chain_info");
    assert_eq!(log_ref.get_warn_dir(), "chain_warn");
    assert_eq!(log_ref.get_error_dir(), "chain_error");
}

#[cfg(test)]
#[tokio::test]
async fn test_log_level_dirs_with_special_characters() {
    let mut log: FileLogger = FileLogger::new("./logs/special", 1_024_000);
    log.set_trace_dir("trace-2024")
        .set_debug_dir("debug_test")
        .set_info_dir("info.logs")
        .set_warn_dir("warn/logs")
        .set_error_dir("error_logs");
    log.trace("special trace message", common_log);
    log.async_trace("async special trace message", common_log)
        .await;
    log.debug("special debug message", common_log);
    log.async_debug("async special debug message", common_log)
        .await;
    log.info("special info message", common_log);
    log.async_info("async special info message", common_log)
        .await;
    log.warn("special warn message", common_log);
    log.async_warn("async special warn message", common_log)
        .await;
    log.error("special error message", common_log);
    log.async_error("async special error message", common_log)
        .await;
}

#[cfg(test)]
#[tokio::test]
async fn test_log_level_dirs_edge_cases() {
    let mut log: FileLogger = FileLogger::new("./logs", 512_000);
    log.set_trace_dir("")
        .set_debug_dir("")
        .set_info_dir("")
        .set_warn_dir("")
        .set_error_dir("");
    assert_eq!(log.get_trace_dir(), "");
    assert_eq!(log.get_debug_dir(), "");
    assert_eq!(log.get_info_dir(), "");
    assert_eq!(log.get_warn_dir(), "");
    assert_eq!(log.get_error_dir(), "");
    log.trace("empty dir trace", common_log);
    log.debug("empty dir debug", common_log);
    log.info("empty dir info", common_log);
    log.warn("empty dir warn", common_log);
    log.error("empty dir error", common_log);
    log.set_trace_dir("valid_trace")
        .set_debug_dir("valid_debug")
        .set_info_dir("valid_info")
        .set_warn_dir("valid_warn")
        .set_error_dir("valid_error");
    let long_dir_name: String = "a".repeat(200);
    log.set_trace_dir(&long_dir_name);
    assert_eq!(log.get_trace_dir().as_str(), long_dir_name.as_str());
}
