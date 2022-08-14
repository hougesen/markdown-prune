mod config;

use config::Config;
use std::{ffi::OsStr, path::PathBuf};

#[derive(Debug)]
struct DeleteResult {
    file_count: u16,
    bytes: u64,
}

fn main() -> std::io::Result<()> {
    let config = Config::new();

    if let Some(start_path) = config.path {
        if !start_path.exists() {
            println!("ERROR: Invalid path");
            return Ok(());
        }

        let result = traverse_dir(start_path, config.delete)?;

        if config.delete {
            println!(
                "Deleted {} files totaling {:.2}mb",
                result.file_count,
                convert_bytes(result.bytes, ByteSize::MB)
            )
        } else {
            println!(
                "Found {} files totaling {:.2}mb",
                result.file_count,
                convert_bytes(result.bytes, ByteSize::MB)
            )
        }
    } else {
        println!("ERROR: Invalid path")
    }

    Ok(())
}

fn traverse_dir(path: PathBuf, delete_files: bool) -> std::io::Result<DeleteResult> {
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

fn check_if_bad_file(file_extension: Option<&OsStr>) -> bool {
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

fn delete_file(path: PathBuf) -> bool {
    match std::fs::remove_file(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

enum ByteSize {
    B,
    KB,
    MB,
    GB,
    TB,
}

fn convert_bytes(bytes: u64, format: ByteSize) -> f64 {
    if bytes > 0 {
        return match format {
            ByteSize::B => (bytes as f64),
            ByteSize::KB => (bytes as f64) / 1000.0,
            ByteSize::MB => (bytes as f64) / 1000.0 / 1000.0,
            ByteSize::GB => (bytes as f64) / 1000.0 / 1000.0 / 1000.0,
            ByteSize::TB => (bytes as f64) / 1000.0 / 1000.0 / 1000.0 / 1000.0,
        };
    }

    0.0
}
