use crate::*;

/// Blanket implementation for any function matching LogFuncTrait signature.
///
/// This allows any compatible closure or function to be used as a log formatter.
impl<F, T> LogFuncTrait<T> for F
where
    F: Fn(T) -> String + Send + Sync,
    T: AsRef<str>,
{
}

/// Default implementation for Log configuration.
impl Default for Log {
    /// Creates default Log configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Default Log instance with default path and file size limit.
    fn default() -> Self {
        Self {
            path: DEFAULT_LOG_DIR.to_owned(),
            limit_file_size: DEFAULT_LOG_FILE_SIZE,
        }
    }
}

impl Log {
    /// Creates new Log configuration with specified parameters.
    ///
    /// # Arguments
    ///
    /// - `P: AsRef<str>` - The path for storing log files, which will be converted to string slice.
    /// - `usize` - The maximum file size limit in bytes.
    ///
    /// # Returns
    ///
    /// - `Self` - A new Log instance with specified configuration.
    #[inline]
    pub fn new<P: AsRef<str>>(path: P, limit_file_size: usize) -> Self {
        Self {
            path: path.as_ref().to_owned(),
            limit_file_size,
        }
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
    pub fn path<P: AsRef<str>>(&mut self, path: P) -> &mut Self {
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
    #[inline]
    pub fn limit_file_size(&mut self, limit_file_size: usize) -> &mut Self {
        self.limit_file_size = limit_file_size;
        self
    }

    /// Checks if logging is enabled.
    ///
    /// # Returns
    ///
    /// - `bool` - True if logging is enabled.
    #[inline]
    pub fn is_enable(&self) -> bool {
        self.limit_file_size != DISABLE_LOG_FILE_SIZE
    }

    /// Checks if logging is disabled.
    ///
    /// # Returns
    ///
    /// - `bool` - True if logging is disabled.
    #[inline]
    pub fn is_disable(&self) -> bool {
        !self.is_enable()
    }

    /// Writes log data synchronously to specified directory.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The data to be logged, which will be converted to string slice.
    /// - `L: LogFuncTrait<T>` - The log formatting function.
    /// - `&str` - The subdirectory for log file.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    fn write_sync<T, L>(&self, data: T, func: L, dir: &str) -> &Self
    where
        T: AsRef<str>,
        L: LogFuncTrait<T>,
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
    /// - `L: LogFuncTrait<T>` - The log formatting function.
    /// - `&str` - The subdirectory for log file.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    async fn write_async<T, L>(&self, data: T, func: L, dir: &str) -> &Self
    where
        T: AsRef<str>,
        L: LogFuncTrait<T>,
    {
        if self.is_disable() {
            return self;
        }
        let out: String = func(data);
        let path: String = get_log_path(dir, &self.path, &self.limit_file_size);
        let _ = async_append_to_file(&path, out.as_bytes()).await;
        self
    }

    /// Logs error message synchronously.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Error data to be logged, which will be converted to string slice.
    /// - `L: LogFuncTrait<T>` - Log formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub fn error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: LogFuncTrait<T>,
    {
        self.write_sync(data, func, ERROR_DIR)
    }

    /// Logs error message asynchronously.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Error data to be logged, which will be converted to string slice.
    /// - `L: LogFuncTrait<T>` - Log formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub async fn async_error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: LogFuncTrait<T>,
    {
        self.write_async(data, func, ERROR_DIR).await
    }

    /// Logs info message synchronously.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Info data to be logged, which will be converted to string slice.
    /// - `L: LogFuncTrait<T>` - Log formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub fn info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: LogFuncTrait<T>,
    {
        self.write_sync(data, func, INFO_DIR)
    }

    /// Logs info message asynchronously.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Info data to be logged, which will be converted to string slice.
    /// - `L: LogFuncTrait<T>` - Log formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub async fn async_info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: LogFuncTrait<T>,
    {
        self.write_async(data, func, INFO_DIR).await
    }

    /// Logs debug message synchronously.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Debug data to be logged, which will be converted to string slice.
    /// - `L: LogFuncTrait<T>` - Log formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub fn debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: LogFuncTrait<T>,
    {
        self.write_sync(data, func, DEBUG_DIR)
    }

    /// Logs debug message asynchronously.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - Debug data to be logged, which will be converted to string slice.
    /// - `L: LogFuncTrait<T>` - Log formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub async fn async_debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: LogFuncTrait<T>,
    {
        self.write_async(data, func, DEBUG_DIR).await
    }
}
