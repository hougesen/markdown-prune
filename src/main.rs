mod config;
mod convert_bytes;
mod directory;

use config::Config;
use convert_bytes::{convert_bytes, ByteSize};
use directory::traverse_dir;
use std::{ffi::OsStr, path::PathBuf};

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
