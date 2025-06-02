use crate::*;

impl<F, T> LogFuncTrait<T> for F
where
    F: Fn(T) -> String + Send + Sync,
    T: ToString,
{
}

impl Default for Log {
    fn default() -> Self {
        Self {
            path: DEFAULT_LOG_DIR.to_owned(),
            limit_file_size: DEFAULT_LOG_FILE_SIZE,
        }
    }
}

impl Log {
    pub fn new<P: ToString>(path: P, limit_file_size: usize) -> Self {
        Self {
            path: path.to_string(),
            limit_file_size,
        }
    }

    pub fn path<P: ToString>(&mut self, path: P) -> &mut Self {
        self.path = path.to_string();
        self
    }

    pub fn limit_file_size(&mut self, limit_file_size: usize) -> &mut Self {
        self.limit_file_size = limit_file_size;
        self
    }

    pub fn is_enable(&self) -> bool {
        self.limit_file_size != DISABLE_LOG_FILE_SIZE
    }

    pub fn is_disable(&self) -> bool {
        !self.is_enable()
    }

    fn write_sync<T, L>(&self, data: T, func: L, dir: &str) -> &Self
    where
        T: ToString,
        L: LogFuncTrait<T>,
    {
        if self.is_disable() {
            return self;
        }
        let out: String = func(data);
        let path: String = get_log_path(dir, &self.path, &self.limit_file_size);
        let _ = append_to_file(&path, &out.as_bytes());
        self
    }

    async fn write_async<T, L>(&self, data: T, func: L, dir: &str) -> &Self
    where
        T: ToString,
        L: LogFuncTrait<T>,
    {
        if self.is_disable() {
            return self;
        }
        let out: String = func(data);
        let path: String = get_log_path(dir, &self.path, &self.limit_file_size);
        let _ = async_append_to_file(&path, &out.as_bytes()).await;
        self
    }

    pub fn error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: ToString,
        L: LogFuncTrait<T>,
    {
        self.write_sync(data, func, ERROR_DIR)
    }

    pub async fn async_error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: ToString,
        L: LogFuncTrait<T>,
    {
        self.write_async(data, func, ERROR_DIR).await
    }

    pub fn info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: ToString,
        L: LogFuncTrait<T>,
    {
        self.write_sync(data, func, INFO_DIR)
    }

    pub async fn async_info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: ToString,
        L: LogFuncTrait<T>,
    {
        self.write_async(data, func, INFO_DIR).await
    }

    pub fn debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: ToString,
        L: LogFuncTrait<T>,
    {
        self.write_sync(data, func, DEBUG_DIR)
    }

    pub async fn async_debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: ToString,
        L: LogFuncTrait<T>,
    {
        self.write_async(data, func, DEBUG_DIR).await
    }
}
