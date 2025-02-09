<center>

## hyperlane-log

[![](https://img.shields.io/crates/v/hyperlane-log.svg)](https://crates.io/crates/hyperlane-log)
[![](https://docs.rs/hyperlane-log/badge.svg)](https://docs.rs/hyperlane-log)
[![](https://github.com/ltpp-universe/hyperlane-log/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/hyperlane-log/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane-log.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/hyperlane-log/)

[Api Docs](https://docs.rs/hyperlane-log/latest/hyperlane_log/)

> A Rust asynchronous logging library that runs on a dedicated thread to avoid blocking other threads. It supports multiple log levels (such as error, info, and debug), and allows custom log handling methods and configuration of log file paths. When a single log file reaches the specified size limit, a new log file will be automatically created.

## Installation

To use this crate, you can run cmd:

```shell
cargo add hyperlane-log
```

## Log Storage Location Description

> Three directories will be created under the user-specified directory: one for error logs, one for info logs, and one for debug logs. Each of these directories will contain a subdirectory named by the date, and the log files within these subdirectories will be named in the format `timestamp.index.log`.

## Use

```rust
use hyperlane_log::*;
let log: Log = Log::new("./logs", 1_024_000, 360);
let log_thread: JoinHandle<()> = log_run(&log);
log.error("error data!", |error| {
    let write_data: String = format!("User error func =>  {:?}\n", error);
    write_data
});
log.info("info data!", |info| {
    let write_data: String = format!("User info func =>  {:?}\n", info);
    write_data
});
log.debug("debug data!", |debug| {
    let write_data: String = format!("User debug func =>  {:#?}\n", debug);
    write_data
});
let _ = log_thread.join();
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
