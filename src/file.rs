const BAD_EXTENSIONS: [&str; 3] = ["md", "markdown", "mkd"];

pub fn check_if_bad_file_ext(file_extension: Option<&std::ffi::OsStr>) -> bool {
    match file_extension {
        Some(ext) => {
            if BAD_EXTENSIONS.contains(&ext.to_str().unwrap_or("")) {
                return true;
            }

            false
        }
        None => false,
    }
}

pub fn check_if_bad_file(file_path: &std::path::Path, bad_files: &Vec<String>) -> bool {
    if !bad_files.is_empty() {
        if let Some(file_name) = file_path.file_name() {
            let file_name_string = file_name.to_string_lossy();

            for bad_file_name in bad_files {
                if file_name.len() < bad_file_name.len() {
                    continue;
                }

                if file_name_string.ends_with(bad_file_name) {
                    return true;
                }
            }
        }
    }

    false
}

pub fn delete_file(path: std::path::PathBuf) -> bool {
    std::fs::remove_file(path).is_ok()
}
