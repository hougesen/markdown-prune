pub fn check_if_bad_file(file_extension: Option<&std::ffi::OsStr>) -> bool {
    match file_extension {
        Some(ext) => {
            if ext == "md" {
                return true;
            }

            false
        }
        None => false,
    }
}

pub fn delete_file(path: std::path::PathBuf) -> bool {
    match std::fs::remove_file(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}
