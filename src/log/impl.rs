use crate::*;

impl Default for Log {
    fn default() -> Self {
        Self {
            path: DEFAULT_LOG_DIR.to_owned(),
            file_size: DEFAULT_LOG_FILE_SIZE,
            interval_millis: DEFAULT_LOG_INTERVAL_MILLIS,
            log_error_queue: Arc::new(RwLock::new(Vec::new())),
            log_info_queue: Arc::new(RwLock::new(Vec::new())),
            log_debug_queue: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl Log {
    pub fn new<T>(path: T, file_size: usize, interval_millis: usize) -> Self
    where
        T: Into<String>,
    {
        Self {
            path: path.into(),
            file_size,
            interval_millis,
            log_error_queue: Arc::new(RwLock::new(Vec::new())),
            log_info_queue: Arc::new(RwLock::new(Vec::new())),
            log_debug_queue: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub(super) fn id_disable(&self) -> bool {
        self.get_file_size() == &DISABLE_LOG_FILE_SIZE
    }

    fn write(list: &mut Vec<(String, ArcLogFunc)>, path: &str) {
        for (log_string, func) in list.iter() {
            let out: String = func(log_string);
            let _ = append_to_file(path, &out.as_bytes());
        }
    }

    fn add_data<T, L>(&self, log_queue: &LogListArcLock, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        if self.id_disable() {
            return self;
        }
        let data_string: String = data.into();
        if let Ok(mut queue) = log_queue.write() {
            queue.push((data_string, Arc::new(func)));
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
        if file_size <= self.file_size {
            return combined_path;
        }
        combined_path_clone.push_str(&self.get_file_name(idx + 1));
        combined_path_clone
    }

    pub(super) fn write_error(&self) {
        let mut list: ListLog = if let Ok(mut error) = self.get_log_error_queue().write() {
            let tmp_error: ListLog = error.drain(..).collect::<Vec<_>>();
            tmp_error
        } else {
            vec![]
        };
        Self::write(&mut list, &self.get_log_path(ERROR_DIR));
    }

    pub(super) fn write_info(&self) {
        let mut list: ListLog = if let Ok(mut info) = self.get_log_info_queue().write() {
            let tmp_info: ListLog = info.drain(..).collect::<Vec<_>>();
            tmp_info
        } else {
            vec![]
        };
        Self::write(&mut list, &self.get_log_path(INFO_DIR));
    }

    pub(super) fn write_debug(&self) {
        let mut list: ListLog = if let Ok(mut debug) = self.get_log_debug_queue().write() {
            let tmp_debug: ListLog = debug.drain(..).collect::<Vec<_>>();
            tmp_debug
        } else {
            vec![]
        };
        Self::write(&mut list, &self.get_log_path(DEBUG_DIR));
    }

    pub fn error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.add_data(self.get_log_error_queue(), data, func);
        self
    }

    pub fn info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.add_data(self.get_log_info_queue(), data, func);
        self
    }

    pub fn debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.add_data(self.get_log_debug_queue(), data, func);
        self
    }
}
