/// Configuration for log file output.
///
/// Contains settings for log file path and size limits.
#[derive(Clone)]
pub struct Log {
    /// Path to the log file.
    pub(super) path: String,
    /// Maximum size limit for log files in bytes.
    pub(super) limit_file_size: usize,
}
