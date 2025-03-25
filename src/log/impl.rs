use crate::*;

impl<'a> Default for Log<'a> {
    fn default() -> Self {
        Self {
            path: DEFAULT_LOG_DIR,
            limit_file_size: DEFAULT_LOG_FILE_SIZE,
        }
    }
}

impl<'a> Log<'a> {
    pub fn new(path: &'a str, limit_file_size: usize) -> Self {
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

    fn write_data<L>(&self, data: &str, func: L, path: String, is_sync: bool) -> &Self
    where
        L: LogFuncTrait,
    {
        if self.is_disable() {
            return self;
        }
        let out: String = func(data);
        if is_sync {
            let _ = r#sync::run_function(move || {
                let _ = append_to_file(&path, &out.as_bytes());
            });
        } else {
            let _ = r#async::run_function(move || async move {
                let _ = async_append_to_file(&path, &out.as_bytes()).await;
            });
        }
        self
    }

    fn common_error<L>(&self, data: &str, func: L, is_sync: bool) -> &Self
    where
        L: LogFuncTrait,
    {
        self.write_data(
            data,
            func,
            get_log_path(ERROR_DIR, self.get_path(), self.get_limit_file_size()),
            is_sync,
        )
    }

    fn common_info<L>(&self, data: &str, func: L, is_sync: bool) -> &Self
    where
        L: LogFuncTrait,
    {
        self.write_data(
            data,
            func,
            get_log_path(INFO_DIR, self.get_path(), self.get_limit_file_size()),
            is_sync,
        )
    }

    fn common_debug<L>(&self, data: &str, func: L, is_sync: bool) -> &Self
    where
        L: LogFuncTrait,
    {
        self.write_data(
            data,
            func,
            get_log_path(DEBUG_DIR, self.get_path(), self.get_limit_file_size()),
            is_sync,
        )
    }

    pub fn error<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.common_error(data, func, true)
    }

    pub fn async_error<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.common_error(data, func, false)
    }

    pub fn info<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.common_info(data, func, true)
    }

    pub fn async_info<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.common_info(data, func, false)
    }

    pub fn debug<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.common_debug(data, func, true)
    }

    pub fn async_debug<L>(&self, data: &str, func: L) -> &Self
    where
        L: LogFuncTrait,
    {
        self.common_debug(data, func, false)
    }
}
