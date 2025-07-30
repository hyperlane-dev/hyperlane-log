/// Default directory path for storing log files.
pub const DEFAULT_LOG_DIR: &str = "./logs";
/// Subdirectory name for error logs.
pub const ERROR_DIR: &str = "error";
/// Subdirectory name for info logs.
pub const INFO_DIR: &str = "info";
/// Subdirectory name for debug logs.
pub const DEBUG_DIR: &str = "debug";
/// File extension for log files.
pub const LOG_EXTENSION: &str = "log";
/// Default starting index number for log files.
pub const DEFAULT_LOG_FILE_START_IDX: usize = 1;
/// Default maximum size limit for log files in bytes.
pub const DEFAULT_LOG_FILE_SIZE: usize = 1_024_000_000;
/// Special value indicating no size limit for log files.
pub const DISABLE_LOG_FILE_SIZE: usize = 0;
/// Root path symbol.
pub(crate) const ROOT_PATH: &str = "/";
/// Dot symbol.
pub(crate) const POINT: &str = ".";
/// Line break symbol.
pub(crate) const BR: &str = "\n";
