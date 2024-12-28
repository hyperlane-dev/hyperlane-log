## hyperlane-log

[![](https://img.shields.io/crates/v/hyperlane-log.svg)](https://crates.io/crates/hyperlane-log)
[![](https://docs.rs/hyperlane-log/badge.svg)](https://docs.rs/hyperlane-log)
[![](https://img.shields.io/crates/l/hyperlane-log.svg)](./LICENSE)
[![](https://github.com/ltpp-universe/hyperlane-log/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/hyperlane-log/actions?query=workflow:Rust)

[Official Documentation](https://docs.ltpp.vip/hyperlane-log/)

[Api Docs](https://docs.rs/hyperlane-log/latest/hyperlane_log/)

> An asynchronous logging library for Rust that operates on a dedicated thread to avoid blocking other threads. Supports multiple log levels (e.g., error, info, debug) and allows custom log handling methods and configurable log file paths.

## Features

## Installation

To use this crate, you can run cmd:

```shell
cargo add hyperlane-log
```

## Use

```rust
use hyperlane_log::*;
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
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
