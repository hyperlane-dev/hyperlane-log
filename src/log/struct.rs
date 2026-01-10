/// Main configuration structure for log file output.
///
/// Controls where logs are stored and their maximum size limits.
/// Use ServerLog::new() to create an instance with custom settings.
#[derive(Clone)]
pub struct ServerLog {
    /// The directory path where log files will be stored.
    pub(super) path: String,
    /// The maximum allowed size (in bytes) for individual log files.
    /// Set to 0 to disable logging.
    pub(super) limit_file_size: usize,
}
