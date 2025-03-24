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

pub(crate) fn get_file_name(idx: usize) -> String {
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

pub(crate) fn get_file_dir_name() -> String {
    format!("{}{}", ROOT_PATH, current_date())
}

pub(crate) fn get_log_path(system_dir: &str, base_path: &str, limit_file_size: &usize) -> String {
    let mut combined_path: String = base_path.trim_end_matches(ROOT_PATH).to_string();
    if !system_dir.starts_with(ROOT_PATH) {
        combined_path.push_str(ROOT_PATH);
    }
    combined_path.push_str(
        system_dir
            .trim_start_matches(ROOT_PATH)
            .trim_end_matches(ROOT_PATH),
    );
    combined_path.push_str(&get_file_dir_name());
    let idx: usize = get_second_element_from_filename(&combined_path);
    let mut combined_path_clone: String = combined_path.clone();
    combined_path.push_str(&get_file_name(idx));
    let file_size: usize = get_file_size(&combined_path).unwrap_or_default() as usize;
    if &file_size <= limit_file_size {
        return combined_path;
    }
    combined_path_clone.push_str(&get_file_name(idx + 1));
    combined_path_clone
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
