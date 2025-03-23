use crate::*;

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

pub fn common_log<T: ToString>(data: &T) -> String {
    format!("{}: {}{}", current_time(), data.to_string(), BR)
}

pub fn log_handler<T: ToString>(log_data: &T) -> String {
    common_log(log_data)
}

pub fn log_debug_handler<T: std::fmt::Debug>(log_data: &T) -> String {
    let write_data: String = format!("{}: {:?}{}", current_time(), log_data, BR);
    write_data.clone()
}

pub fn log_debug_format_handler<T: std::fmt::Debug>(log_data: &T) -> String {
    let write_data: String = format!("{}: {:#?}{}", current_time(), log_data, BR);
    write_data.clone()
}
