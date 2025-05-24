<center>

## hyperlane-log

[![](https://img.shields.io/crates/v/hyperlane-log.svg)](https://crates.io/crates/hyperlane-log)
[![](https://img.shields.io/crates/d/hyperlane-log.svg)](https://img.shields.io/crates/d/hyperlane-log.svg)
[![](https://docs.rs/hyperlane-log/badge.svg)](https://docs.rs/hyperlane-log)
[![](https://github.com/eastspire/hyperlane-log/workflows/Rust/badge.svg)](https://github.com/eastspire/hyperlane-log/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane-log.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/hyperlane-log/)

[Api Docs](https://docs.rs/hyperlane-log/latest/hyperlane_log/)

> A Rust logging library that supports both asynchronous and synchronous logging. It provides multiple log levels, such as error, info, and debug. Users can define custom log handling methods and configure log file paths. The library supports log rotation, automatically creating a new log file when the current file reaches the specified size limit. It allows flexible logging configurations, making it suitable for both high-performance asynchronous applications and traditional synchronous logging scenarios. The asynchronous mode utilizes Tokio's async channels for efficient log buffering, while the synchronous mode writes logs directly to the file system.

## Installation

To use this crate, you can run cmd:

```shell
cargo add hyperlane-log
```

## Log Storage Location Description

> Three directories will be created under the user-specified directory: one for error logs, one for info logs, and one for debug logs. Each of these directories will contain a subdirectory named by the date, and the log files within these subdirectories will be named in the format `timestamp.index.log`.

## Use sync

```rust
use hyperlane_log::*;

let log: Log = Log::new("./logs", 1_024_000);
log.error("error data!", |error| {
    let write_data: String = format!("User error func => {:?}\n", error);
    write_data
});
log.error(String::from("error data!"), |error| {
    let write_data: String = format!("User error func => {:?}\n", error);
    write_data
});
log.info("info data!", |info| {
    let write_data: String = format!("User info func => {:?}\n", info);
    write_data
});
log.info(String::from("info data!"), |info| {
    let write_data: String = format!("User info func => {:?}\n", info);
    write_data
});
log.debug("debug data!", |debug| {
    let write_data: String = format!("User debug func => {:#?}\n", debug);
    write_data
});
log.debug(String::from("debug data!"), |debug| {
    let write_data: String = format!("User debug func => {:#?}\n", debug);
    write_data
});
```

## Use async

```rust
use hyperlane_log::*;

let log: Log = Log::new("./logs", 1_024_000);
log.async_error("async error data!", |error| {
    let write_data: String = format!("User error func => {:?}\n", error);
    write_data
}).await;
log.async_error(String::from("async error data!"), |error| {
    let write_data: String = format!("User error func => {:?}\n", error);
    write_data
}).await;
log.async_info("async info data!", |info| {
    let write_data: String = format!("User info func => {:?}\n", info);
    write_data
}).await;
log.async_info(String::from("async info data!"), |info| {
    let write_data: String = format!("User info func => {:?}\n", info);
    write_data
}).await;
log.async_debug("async debug data!", |debug| {
    let write_data: String = format!("User debug func => {:#?}\n", debug);
    write_data
}).await;
log.async_debug(String::from("async debug data!"), |debug| {
    let write_data: String = format!("User debug func => {:#?}\n", debug);
    write_data
}).await;
```

## Disable log

```rust
let log: Log = Log::new("./logs", DISABLE_LOG_FILE_SIZE);
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
