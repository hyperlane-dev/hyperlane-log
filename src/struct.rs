/// Main configuration structure for log file output.
///
/// Controls where logs are stored and their maximum size limits.
/// Use FileLogger::new() to create an instance with custom settings.
#[derive(Clone)]
pub struct FileLogger {
    /// The directory path where log files will be stored.
    pub(super) path: String,
    /// The maximum allowed size (in bytes) for individual log files.
    /// Set to 0 to disable logging.
    pub(super) limit_file_size: usize,
    /// Subdirectory name for trace logs.
    pub(super) trace_dir: String,
    /// Subdirectory name for debug logs.
    pub(super) debug_dir: String,
    /// Subdirectory name for info logs.
    pub(super) info_dir: String,
    /// Subdirectory name for warn logs.
    pub(super) warn_dir: String,
    /// Subdirectory name for error logs.
    pub(super) error_dir: String,
}
