use crate::file::{check_if_bad_file, delete_file};

#[derive(Debug)]
pub struct DeleteResult {
    pub file_count: u16,
    pub bytes: u64,
}

pub fn traverse_dir(path: std::path::PathBuf, delete_files: bool) -> std::io::Result<DeleteResult> {
    let mut result = DeleteResult {
        file_count: 0,
        bytes: 0,
    };

    for entry in std::fs::read_dir(&path)? {
        let entry_path = entry?.path();

        if entry_path.is_dir() {
            let path_result = traverse_dir(entry_path, delete_files)?;

            result.file_count += path_result.file_count;
            result.bytes += path_result.bytes;
        } else if entry_path.is_file() {
            let file_extension = entry_path.extension();
            let bad_file = check_if_bad_file(file_extension);

            if bad_file {
                result.file_count += 1;
                let file_size = entry_path.metadata()?.len();

                if file_size > 0 {
                    result.bytes += file_size;
                }

                if delete_files {
                    delete_file(entry_path);
                }
            }
        }
    }

    Ok(result)
}
