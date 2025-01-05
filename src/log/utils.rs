use super::constant::{DEFAULT_LOG_FILE_START_IDX, POINT};
use std::{
    fs::{self, metadata, read_dir, OpenOptions},
    io::Write,
};

#[inline]
pub fn write_to_file(file_path: &str, content: &[u8]) {
    if let Some(parent_dir) = std::path::Path::new(file_path).parent() {
        let _ = fs::create_dir_all(parent_dir);
    }
    let _ = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)
        .and_then(|mut file| {
            let _ = file.write_all(content);
            Ok(())
        });
}

#[inline]
pub fn get_file_size(file_path: &str) -> usize {
    metadata(file_path)
        .and_then(|metadata| Ok(metadata.len()))
        .unwrap_or_default() as usize
}

#[inline]
pub(crate) fn get_second_element_from_filename(dir_path: &str) -> usize {
    let mut res_idx: usize = DEFAULT_LOG_FILE_START_IDX;
    if let Ok(entries) = read_dir(dir_path) {
        for entry in entries.filter_map(Result::ok) {
            let file_name: String = entry.file_name().to_string_lossy().to_string();
            let parts: Vec<&str> = file_name.split(POINT).collect();
            if parts.len() > 1 {
                if let Ok(second_element) = parts[1].parse::<usize>() {
                    res_idx = second_element.max(res_idx);
                }
            }
        }
    }
    res_idx.max(DEFAULT_LOG_FILE_START_IDX)
}
