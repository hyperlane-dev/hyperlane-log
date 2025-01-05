use super::{
    constant::{
        DEBUG_DIR, DEFAULT_LOG_DIR, DEFAULT_LOG_FILE_SIZE, ERROR_DIR, INFO_DIR, LOG_EXTION, POINT,
        SLASH,
    },
    r#trait::{LogDataTrait, LogFuncTrait},
    r#type::{Log, LogArcLock},
    utils::{get_file_size, get_second_element_from_filename, write_to_file},
};
use crate::BoxLogFunc;
use hyperlane_time::*;
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

lazy_static! {
    static ref LOG_ERROR_QUEUE: LogArcLock = Arc::new(RwLock::new(Vec::new()));
    static ref LOG_INFO_QUEUE: LogArcLock = Arc::new(RwLock::new(Vec::new()));
    static ref LOG_DEBUG_QUEUE: LogArcLock = Arc::new(RwLock::new(Vec::new()));
}

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
            write_to_file(path, &out.as_bytes());
        }
        list.clear();
    }

    #[inline]
    fn add_data<T, L>(log_queue: &LogArcLock, data: T, func: L)
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        if let Ok(mut queue) = log_queue.write() {
            let data_string: String = data.into();
            queue.push((data_string, Box::new(func)));
        }
    }

    #[inline]
    fn get_file_name(&self, idx: usize) -> String {
        format!(
            "{}{}{}{}{}{}",
            SLASH,
            current_date(),
            POINT,
            idx,
            POINT,
            LOG_EXTION
        )
    }

    #[inline]
    fn get_file_dir_name(&self) -> String {
        format!("{}{}", SLASH, current_date())
    }

    #[inline]
    fn get_log_path(&self, system_dir: &str) -> String {
        let base_path: &String = self.get_path();
        let mut combined_path: String = base_path.trim_end_matches(SLASH).to_string();
        if !system_dir.starts_with(SLASH) {
            combined_path.push_str(SLASH);
        }
        combined_path.push_str(system_dir.trim_start_matches(SLASH).trim_end_matches(SLASH));
        combined_path.push_str(&self.get_file_dir_name());
        let idx: usize = get_second_element_from_filename(&combined_path);
        let mut combined_path_clone: String = combined_path.clone();
        combined_path.push_str(&self.get_file_name(idx));
        let file_size: usize = get_file_size(&combined_path);
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
        }
    }

    #[inline]
    pub(super) fn write_info(&self) {
        if let Ok(mut info) = LOG_INFO_QUEUE.write() {
            Self::write(&mut *info, &self.get_log_path(INFO_DIR));
        }
    }

    #[inline]
    pub(super) fn write_debug(&self) {
        if let Ok(mut debug) = LOG_DEBUG_QUEUE.write() {
            Self::write(&mut *debug, &self.get_log_path(DEBUG_DIR));
        }
    }

    #[inline]
    pub fn log_error<T, L>(&self, data: T, func: L)
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        Self::add_data(&LOG_ERROR_QUEUE, data, func);
    }

    #[inline]
    pub fn log_info<T, L>(&self, data: T, func: L)
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        Self::add_data(&LOG_INFO_QUEUE, data, func);
    }

    #[inline]
    pub fn log_debug<T, L>(&self, data: T, func: L)
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        Self::add_data(&LOG_DEBUG_QUEUE, data, func);
    }
}
