use crate::file::{check_if_bad_file, check_if_bad_file_ext, delete_file};

pub struct DeleteResult {
    pub file_count: u64,
    pub bytes: u64,
}

pub fn traverse_dir(
    path: &std::path::Path,
    delete_files: bool,
    custom_bad_files: &Vec<String>,
) -> std::io::Result<DeleteResult> {
    let mut result = DeleteResult {
        file_count: 0,
        bytes: 0,
    };

    for entry in std::fs::read_dir(path)? {
        let entry_path = if entry.is_ok() {
            entry?.path()
        } else {
            continue;
        };

        if entry_path.is_dir() {
            if let Ok(path_result) = traverse_dir(&entry_path, delete_files, custom_bad_files) {
                result.file_count += path_result.file_count;
                result.bytes += path_result.bytes;
            }
        } else if entry_path.is_file()
            && (check_if_bad_file_ext(entry_path.extension())
                || (!custom_bad_files.is_empty()
                    && check_if_bad_file(&entry_path, custom_bad_files)))
        {
            result.file_count += 1;

            if let Ok(metadata) = entry_path.metadata() {
                result.bytes += metadata.len();
            }

            if delete_files {
                delete_file(entry_path);
            }
        }
    }

    Ok(result)
}
