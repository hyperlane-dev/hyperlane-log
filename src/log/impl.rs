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
    pub fn new<T>(path: T, limit_file_size: usize) -> Self
    where
        T: Into<String>,
    {
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

    fn write_data<T, L>(&self, data: T, func: L, path: String, is_sync: bool) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        if self.is_disable() {
            return self;
        }
        let data_string: String = data.into();
        let out: String = func(&data_string);
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

    fn common_error<T, L>(&self, data: T, func: L, is_sync: bool) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.write_data(
            data,
            func,
            get_log_path(ERROR_DIR, self.get_path(), self.get_limit_file_size()),
            is_sync,
        )
    }

    fn common_info<T, L>(&self, data: T, func: L, is_sync: bool) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.write_data(
            data,
            func,
            get_log_path(INFO_DIR, self.get_path(), self.get_limit_file_size()),
            is_sync,
        )
    }

    fn common_debug<T, L>(&self, data: T, func: L, is_sync: bool) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.write_data(
            data,
            func,
            get_log_path(DEBUG_DIR, self.get_path(), self.get_limit_file_size()),
            is_sync,
        )
    }

    pub fn error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.common_error(data, func, true)
    }

    pub fn async_error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.common_error(data, func, false)
    }

    pub fn info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.common_info(data, func, true)
    }

    pub fn async_info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.common_info(data, func, false)
    }

    pub fn debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.common_debug(data, func, true)
    }

    pub fn async_debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.common_debug(data, func, false)
    }
}
