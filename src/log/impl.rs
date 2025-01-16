use super::{
    constant::*,
    r#trait::{LogDataTrait, LogFuncTrait},
    r#type::{Log, LogListArcLock},
    utils::*,
};
use crate::BoxLogFunc;
use file_operation::*;
use http_type::*;
use hyperlane_time::*;
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

static LOG_ERROR_QUEUE: Lazy<LogListArcLock> = Lazy::new(|| Arc::new(RwLock::new(Vec::new())));
static LOG_INFO_QUEUE: Lazy<LogListArcLock> = Lazy::new(|| Arc::new(RwLock::new(Vec::new())));
static LOG_DEBUG_QUEUE: Lazy<LogListArcLock> = Lazy::new(|| Arc::new(RwLock::new(Vec::new())));

impl Default for Log {
    #[inline]
    fn default() -> Self {
        Self {
            path: DEFAULT_LOG_DIR.to_owned(),
            file_size: DEFAULT_LOG_FILE_SIZE,
        }
    }
}

impl Log {
    #[inline]
    pub fn new<T>(path: T, file_size: usize) -> Self
    where
        T: Into<String>,
    {
        Self {
            path: path.into(),
            file_size,
        }
    }

    #[inline]
    fn write(list: &mut Vec<(String, BoxLogFunc)>, path: &str) {
        for (log_string, func) in list.iter() {
            let out: String = func(log_string);
            let _ = write_to_file(path, &out.as_bytes());
        }
    }

    #[inline]
    fn add_data<T, L>(log_queue: &LogListArcLock, data: T, func: L)
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        let data_string: String = data.into();
        {
            if let Ok(mut queue) = log_queue.write() {
                queue.push((data_string, Box::new(func)));
            }
        }
    }

    #[inline]
    fn get_file_name(&self, idx: usize) -> String {
        format!(
            "{}{}{}{}{}{}",
            ROOT_PATH,
            current_date(),
            POINT,
            idx,
            POINT,
            LOG_EXTION
        )
    }

    #[inline]
    fn get_file_dir_name(&self) -> String {
        format!("{}{}", ROOT_PATH, current_date())
    }

    #[inline]
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

    #[inline]
    pub(super) fn write_error(&self) {
        if let Ok(mut error) = LOG_ERROR_QUEUE.write() {
            Self::write(&mut *error, &self.get_log_path(ERROR_DIR));
            error.clear();
        }
    }

    #[inline]
    pub(super) fn write_info(&self) {
        if let Ok(mut info) = LOG_INFO_QUEUE.write() {
            Self::write(&mut *info, &self.get_log_path(INFO_DIR));
            info.clear();
        }
    }

    #[inline]
    pub(super) fn write_debug(&self) {
        if let Ok(mut debug) = LOG_DEBUG_QUEUE.write() {
            Self::write(&mut *debug, &self.get_log_path(DEBUG_DIR));
            debug.clear();
        }
    }

    #[inline]
    pub fn error<T, L>(&self, data: T, func: L)
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        Self::add_data(&LOG_ERROR_QUEUE, data, func);
    }

    #[inline]
    pub fn info<T, L>(&self, data: T, func: L)
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        Self::add_data(&LOG_INFO_QUEUE, data, func);
    }

    #[inline]
    pub fn debug<T, L>(&self, data: T, func: L)
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        Self::add_data(&LOG_DEBUG_QUEUE, data, func);
    }
}
