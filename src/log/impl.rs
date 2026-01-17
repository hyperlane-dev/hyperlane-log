use crate::*;

/// Blanket implementation for any function matching FileLoggerFuncTrait signature.
///
/// This allows any compatible closure or function to be used as a log formatter.
impl<F, T> FileLoggerFuncTrait<T> for F
where
    F: Fn(T) -> String + Send + Sync,
    T: AsRef<str>,
{
}

/// Default implementation for FileLogger configuration.
impl Default for FileLogger {
    /// Creates default FileLogger configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Default FileLogger instance with default path and file size limit.
    #[inline(always)]
    fn default() -> Self {
        Self {
            path: DEFAULT_LOG_DIR.to_owned(),
            limit_file_size: DEFAULT_LOG_FILE_SIZE,
            trace_dir: TRACE_DIR.to_owned(),
            debug_dir: DEBUG_DIR.to_owned(),
            info_dir: INFO_DIR.to_owned(),
            warn_dir: WARN_DIR.to_owned(),
            error_dir: ERROR_DIR.to_owned(),
        }
    }
}

impl FileLogger {
    /// Creates new FileLogger configuration with specified parameters.
    ///
    /// # Arguments
    ///
    /// - `P: AsRef<str>` - The path for storing log files, which will be converted to string slice.
    /// - `usize` - The maximum file size limit in bytes.
    ///
    /// # Returns
    ///
    /// - `Self` - A new FileLogger instance with specified configuration.
    #[inline(always)]
    pub fn new<P: AsRef<str>>(path: P, limit_file_size: usize) -> Self {
        Self {
            path: path.as_ref().to_owned(),
            limit_file_size,
            trace_dir: TRACE_DIR.to_owned(),
            debug_dir: DEBUG_DIR.to_owned(),
            info_dir: INFO_DIR.to_owned(),
            warn_dir: WARN_DIR.to_owned(),
            error_dir: ERROR_DIR.to_owned(),
        }
    }

    /// Gets the log file storage path.
    ///
    /// # Returns
    ///
    /// - `&String` - Reference to the log file storage path.
    #[inline(always)]
    pub fn get_path(&self) -> &String {
        &self.path
    }

    /// Gets the maximum allowed size (in bytes) for individual log files.
    ///
    /// # Returns
    ///
    /// - `&usize` - Reference to the maximum file size limit in bytes.
    #[inline(always)]
    pub fn get_limit_file_size(&self) -> &usize {
        &self.limit_file_size
    }

    /// Gets the trace log directory name.
    ///
    /// # Returns
    ///
    /// - `&String` - Reference to the trace log directory name.
    #[inline(always)]
    pub fn get_trace_dir(&self) -> &String {
        &self.trace_dir
    }

    /// Gets the debug log directory name.
    ///
    /// # Returns
    ///
    /// - `&String` - Reference to the debug log directory name.
    #[inline(always)]
    pub fn get_debug_dir(&self) -> &String {
        &self.debug_dir
    }

    /// Gets the info log directory name.
    ///
    /// # Returns
    ///
    /// - `&String` - Reference to the info log directory name.
    #[inline(always)]
    pub fn get_info_dir(&self) -> &String {
        &self.info_dir
    }

    /// Gets the warn log directory name.
    ///
    /// # Returns
    ///
    /// - `&String` - Reference to the warn log directory name.
    #[inline(always)]
    pub fn get_warn_dir(&self) -> &String {
        &self.warn_dir
    }

    /// Gets the error log directory name.
    ///
    /// # Returns
    ///
    /// - `&String` - Reference to the error log directory name.
    #[inline(always)]
    pub fn get_error_dir(&self) -> &String {
        &self.error_dir
    }

    /// Sets the log file storage path.
    ///
    /// # Arguments
    ///
    /// - `P: AsRef<str>` - The new path for storing log files, which will be converted to string slice.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for method chaining.
    #[inline(always)]
    pub fn set_path<P: AsRef<str>>(&mut self, path: P) -> &mut Self {
        self.path = path.as_ref().to_owned();
        self
    }

    /// Sets the maximum size limit for log files.
    ///
    /// # Arguments
    ///
    /// - `usize` - The new maximum file size limit in bytes.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for method chaining.
    #[inline(always)]
    pub fn set_limit_file_size(&mut self, limit_file_size: usize) -> &mut Self {
        self.limit_file_size = limit_file_size;
        self
    }

    /// Sets the trace log directory name.
    ///
    /// # Arguments
    ///
    /// - `P: AsRef<str>` - The directory name for trace logs.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for method chaining.
    #[inline(always)]
    pub fn set_trace_dir<P: AsRef<str>>(&mut self, dir: P) -> &mut Self {
        self.trace_dir = dir.as_ref().to_owned();
        self
    }

    /// Sets the debug log directory name.
    ///
    /// # Arguments
    ///
    /// - `P: AsRef<str>` - The directory name for debug logs.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for method chaining.
    #[inline(always)]
    pub fn set_debug_dir<P: AsRef<str>>(&mut self, dir: P) -> &mut Self {
        self.debug_dir = dir.as_ref().to_owned();
        self
    }

    /// Sets the info log directory name.
    ///
    /// # Arguments
    ///
    /// - `P: AsRef<str>` - The directory name for info logs.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for method chaining.
    #[inline(always)]
    pub fn set_info_dir<P: AsRef<str>>(&mut self, dir: P) -> &mut Self {
        self.info_dir = dir.as_ref().to_owned();
        self
    }

    /// Sets the warn log directory name.
    ///
    /// # Arguments
    ///
    /// - `P: AsRef<str>` - The directory name for warn logs.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for method chaining.
    #[inline(always)]
    pub fn set_warn_dir<P: AsRef<str>>(&mut self, dir: P) -> &mut Self {
        self.warn_dir = dir.as_ref().to_owned();
        self
    }

    /// Sets the error log directory name.
    ///
    /// # Arguments
    ///
    /// - `P: AsRef<str>` - The directory name for error logs.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for method chaining.
    #[inline(always)]
    pub fn set_error_dir<P: AsRef<str>>(&mut self, dir: P) -> &mut Self {
        self.error_dir = dir.as_ref().to_owned();
        self
    }

    /// Checks if logging is enabled.
    ///
    /// # Returns
    ///
    /// - `bool` - True if logging is enabled.
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        self.limit_file_size != DISABLE_LOG_FILE_SIZE
    }

    /// Checks if logging is disabled.
    ///
    /// # Returns
    ///
    /// - `bool` - True if logging is disabled.
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        !self.is_enable()
    }

    /// Writes log data synchronously to specified directory.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The data to be logged, which will be converted to string slice.
    /// - `L: FileLoggerFuncTrait<T>` - The log formatting function.
    /// - `&str` - The subdirectory for log file.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    fn write_sync<T, L>(&self, data: T, func: L, dir: &str) -> &Self
    where
        T: AsRef<str>,
        L: FileLoggerFuncTrait<T>,
    {
        if self.is_disable() {
            return self;
        }
        let out: String = func(data);
        let path: String = get_log_path(dir, &self.path, &self.limit_file_size);
        let _ = append_to_file(&path, out.as_bytes());
        self
    }

    /// Writes log data asynchronously to specified directory.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The data to be logged, which will be converted to string slice.
    /// - `L: FileLoggerFuncTrait<T>` - The log formatting function.
    /// - `&str` - The subdirectory for log file.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    async fn write_async<T, L>(&self, data: T, func: L, dir: &str) -> &Self
    where
        T: AsRef<str>,
        L: FileLoggerFuncTrait<T>,
    {
        if self.is_disable() {
            return self;
        }
        let out: String = func(data);
        let path: String = get_log_path(dir, &self.path, &self.limit_file_size);
        let _ = async_append_to_file(&path, out.as_bytes()).await;
        self
    }

    /// Logs trace message synchronously.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Trace data to be logged, which will be converted to string slice.
    /// - `L: FileLoggerFuncTrait<T>` - FileLogger formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub fn trace<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: FileLoggerFuncTrait<T>,
    {
        self.write_sync(data, func, &self.trace_dir)
    }

    /// Logs trace message asynchronously.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Trace data to be logged, which will be converted to string slice.
    /// - `L: FileLoggerFuncTrait<T>` - FileLogger formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub async fn async_trace<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: FileLoggerFuncTrait<T>,
    {
        self.write_async(data, func, &self.trace_dir).await
    }

    /// Logs debug message synchronously.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Debug data to be logged, which will be converted to string slice.
    /// - `L: FileLoggerFuncTrait<T>` - FileLogger formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub fn debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: FileLoggerFuncTrait<T>,
    {
        self.write_sync(data, func, &self.debug_dir)
    }

    /// Logs debug message asynchronously.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Debug data to be logged, which will be converted to string slice.
    /// - `L: FileLoggerFuncTrait<T>` - FileLogger formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub async fn async_debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: FileLoggerFuncTrait<T>,
    {
        self.write_async(data, func, &self.debug_dir).await
    }

    /// Logs info message synchronously.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Info data to be logged, which will be converted to string slice.
    /// - `L: FileLoggerFuncTrait<T>` - FileLogger formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub fn info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: FileLoggerFuncTrait<T>,
    {
        self.write_sync(data, func, &self.info_dir)
    }

    /// Logs info message asynchronously.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Info data to be logged, which will be converted to string slice.
    /// - `L: FileLoggerFuncTrait<T>` - FileLogger formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub async fn async_info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: FileLoggerFuncTrait<T>,
    {
        self.write_async(data, func, &self.info_dir).await
    }

    /// Logs warn message synchronously.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Warn data to be logged, which will be converted to string slice.
    /// - `L: FileLoggerFuncTrait<T>` - FileLogger formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub fn warn<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: FileLoggerFuncTrait<T>,
    {
        self.write_sync(data, func, &self.warn_dir)
    }

    /// Logs warn message asynchronously.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Warn data to be logged, which will be converted to string slice.
    /// - `L: FileLoggerFuncTrait<T>` - FileLogger formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub async fn async_warn<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: FileLoggerFuncTrait<T>,
    {
        self.write_async(data, func, &self.warn_dir).await
    }

    /// Logs error message synchronously.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Error data to be logged, which will be converted to string slice.
    /// - `L: FileLoggerFuncTrait<T>` - FileLogger formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub fn error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: FileLoggerFuncTrait<T>,
    {
        self.write_sync(data, func, &self.error_dir)
    }

    /// Logs error message asynchronously.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Error data to be logged, which will be converted to string slice.
    /// - `L: FileLoggerFuncTrait<T>` - FileLogger formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub async fn async_error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: FileLoggerFuncTrait<T>,
    {
        self.write_async(data, func, &self.error_dir).await
    }
}
