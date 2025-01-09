use super::constant::*;
use http_type::*;
use std::{
    fmt,
    fs::{self, metadata, read_dir, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

#[derive(Debug, Clone)]
pub struct DataString(String);

impl From<Vec<u8>> for DataString {
    #[inline]
    fn from(bytes: Vec<u8>) -> Self {
        DataString(String::from_utf8(bytes).unwrap_or_else(|_| String::new()))
    }
}

impl fmt::Display for DataString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[inline]
pub fn read_from_file<T>(file_path: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: From<Vec<u8>>,
{
    let path: &Path = Path::new(file_path);
    let mut file: File = File::open(path)?;
    let mut content: Vec<u8> = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(T::from(content))
}

#[inline]
pub fn write_to_file(file_path: &str, content: &[u8]) -> Result<(), std::io::Error> {
    if let Some(parent_dir) = std::path::Path::new(file_path).parent() {
        let _ = fs::create_dir_all(parent_dir);
    }
    OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)
        .and_then(|mut file| file.write_all(content))
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
