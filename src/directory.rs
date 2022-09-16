use crate::file::{check_if_bad_file, check_if_bad_file_ext, delete_file};

#[derive(Debug)]
pub struct DeleteResult {
    pub file_count: u16,
    pub bytes: u64,
}

pub fn traverse_dir(
    path: std::path::PathBuf,
    delete_files: bool,
    custom_bad_files: &Vec<String>,
) -> std::io::Result<DeleteResult> {
    let mut result = DeleteResult {
        file_count: 0,
        bytes: 0,
    };

    for entry in std::fs::read_dir(&path)? {
        let entry_path = entry?.path();

        if entry_path.is_dir() {
            let path_result = traverse_dir(entry_path, delete_files, custom_bad_files)?;

            result.file_count += path_result.file_count;
            result.bytes += path_result.bytes;
        } else if entry_path.is_file() {
            let file_extension = entry_path.extension();

            if check_if_bad_file_ext(file_extension)
                || (!custom_bad_files.is_empty()
                    && check_if_bad_file(&entry_path, custom_bad_files))
            {
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
