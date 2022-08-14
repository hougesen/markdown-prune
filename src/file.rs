const BAD_EXTENSIONS: [&str; 3] = ["md", "markdown", "mkd"];

pub fn check_if_bad_file(file_extension: Option<&std::ffi::OsStr>) -> bool {
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

pub fn delete_file(path: std::path::PathBuf) -> bool {
    std::fs::remove_file(path).is_ok()
}
