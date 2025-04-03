use crate::*;

impl Default for Log {
    fn default() -> Self {
        Self {
            path: DEFAULT_LOG_DIR.to_owned(),
            limit_file_size: DEFAULT_LOG_FILE_SIZE,
        }
    }
}

impl Log {
    pub fn new(path: &str, limit_file_size: usize) -> Self {
        Self {
            path: path.into(),
            limit_file_size,
        }
    }

    pub fn is_enable(&self) -> bool {
        self.get_limit_file_size() != &DISABLE_LOG_FILE_SIZE
    }

    pub fn is_disable(&self) -> bool {
        !self.is_enable()
    }

    fn write_sync<L>(&self, data: &str, func: L, dir: &str) -> &Self
    where
        L: LogFuncTrait,
    {
        if self.is_disable() {
            return self;
        }
        let out: String = func(data);
        let path = get_log_path(dir, self.get_path(), self.get_limit_file_size());
        let _ = append_to_file(&path, &out.as_bytes());
        self
    }

    async fn write_async<L>(&self, data: &str, func: L, dir: &str) -> &Self
    where
        L: LogFuncTrait,
    {
        if self.is_disable() {
            return self;
        }
        let out: String = func(data);
        let path = get_log_path(dir, self.get_path(), self.get_limit_file_size());
        let _ = async_append_to_file(&path, &out.as_bytes()).await;
        self
    }

    pub fn error<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.write_sync(data, func, ERROR_DIR)
    }

    pub async fn async_error<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.write_async(data, func, ERROR_DIR).await
    }

    pub fn info<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.write_sync(data, func, INFO_DIR)
    }

    pub async fn async_info<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.write_async(data, func, INFO_DIR).await
    }

    pub fn debug<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.write_sync(data, func, DEBUG_DIR)
    }

    pub async fn async_debug<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.write_async(data, func, DEBUG_DIR).await
    }
}
