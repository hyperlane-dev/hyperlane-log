use crate::*;

impl Default for Log {
    fn default() -> Self {
        Self {
            path: DEFAULT_LOG_DIR.to_owned(),
            file_size: DEFAULT_LOG_FILE_SIZE,
            log_error_queue: Arc::new(RwLock::new(Vec::new())),
            log_info_queue: Arc::new(RwLock::new(Vec::new())),
            log_debug_queue: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl Log {
    pub fn new<T>(path: T, file_size: usize) -> Self
    where
        T: Into<String>,
    {
        Self {
            path: path.into(),
            file_size,
            log_error_queue: Arc::new(RwLock::new(Vec::new())),
            log_info_queue: Arc::new(RwLock::new(Vec::new())),
            log_debug_queue: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn is_enable(&self) -> bool {
        self.get_file_size() != &DISABLE_LOG_FILE_SIZE
    }

    pub fn is_disable(&self) -> bool {
        !self.is_enable()
    }

    fn add_data<T, L>(&self, data: T, func: L, path: String, is_sync: bool) -> &Self
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

    fn get_file_name(&self, idx: usize) -> String {
        format!(
            "{}{}{}{}{}{}",
            ROOT_PATH,
            current_date(),
            POINT,
            idx,
            POINT,
            LOG_EXTENSION
        )
    }

    fn get_file_dir_name(&self) -> String {
        format!("{}{}", ROOT_PATH, current_date())
    }

    fn get_log_path(&self, system_dir: &str) -> String {
        let base_path: &String = self.get_path();
        let mut combined_path: String = base_path.trim_end_matches(ROOT_PATH).to_string();
        if !system_dir.starts_with(ROOT_PATH) {
            combined_path.push_str(ROOT_PATH);
        }
        combined_path.push_str(
            system_dir
                .trim_start_matches(ROOT_PATH)
                .trim_end_matches(ROOT_PATH),
        );
        combined_path.push_str(&self.get_file_dir_name());
        let idx: usize = get_second_element_from_filename(&combined_path);
        let mut combined_path_clone: String = combined_path.clone();
        combined_path.push_str(&self.get_file_name(idx));
        let file_size: usize = get_file_size(&combined_path).unwrap_or_default() as usize;
        if &file_size <= self.get_file_size() {
            return combined_path;
        }
        combined_path_clone.push_str(&self.get_file_name(idx + 1));
        combined_path_clone
    }

    pub fn error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.add_data(data, func, self.get_log_path(ERROR_DIR), true);
        self
    }

    pub fn async_error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.add_data(data, func, self.get_log_path(ERROR_DIR), false);
        self
    }

    pub fn info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.add_data(data, func, self.get_log_path(INFO_DIR), true);
        self
    }

    pub fn async_info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.add_data(data, func, self.get_log_path(INFO_DIR), false);
        self
    }

    pub fn debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.add_data(data, func, self.get_log_path(DEBUG_DIR), true);
        self
    }

    pub fn async_debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.add_data(data, func, self.get_log_path(DEBUG_DIR), false);
        self
    }
}
