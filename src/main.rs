use std::{ffi::OsStr, path::Path};

#[derive(Debug)]
struct DeleteResult {
    file_count: u16,
    bytes: u64,
}

fn main() -> std::io::Result<()> {
    let path_arg = std::env::args().nth(1);

    if let Some(path_arg) = path_arg {
        let start_path = Path::new(&path_arg);

        if !start_path.exists() {
            println!("ERROR: Path does not exist");
            return Ok(());
        }

        let delete_result = traverse_dir(start_path)?;

        println!(
            "Deleted {} files totaling {:.2}mb",
            delete_result.file_count,
            convert_bytes(delete_result.bytes, ByteSize::MB)
        )
    } else {
        println!("ERROR: Missing path")
    }

    Ok(())
}

fn traverse_dir(path: &Path) -> std::io::Result<DeleteResult> {
    let mut result = DeleteResult {
        file_count: 0,
        bytes: 0,
    };

    for entry in std::fs::read_dir(&path)? {
        let entry_path = entry?.path();

        if entry_path.is_dir() {
            let path_result = traverse_dir(entry_path.as_path())?;

            result.file_count += path_result.file_count;
            result.bytes += path_result.bytes;
        } else if entry_path.is_file() {
            let file_extension = entry_path.extension();
            let bad_file = check_if_bad_file(file_extension);

            if bad_file {
                result.file_count += 1;
                let file_size = entry_path.metadata()?.len();

                if file_size > 0 {
                    println!("Current bytes found: {}", file_size);

                    result.bytes += file_size;
                }

                println!(
                    "Bad file len: {} {:#?}",
                    entry_path.metadata()?.len(),
                    entry_path
                );

                delete_file(entry_path.as_path());
            }
        }
    }

    Ok(result)
}

fn check_if_bad_file(file_extension: Option<&OsStr>) -> bool {
    return match file_extension {
        Some(ext) => {
            if ext == "md" {
                return true;
            }

            return false;
        }
        None => false,
    };
}

fn delete_file(path: &Path) -> bool {
    println!("DELETE FILE {:?}", path);

    return match std::fs::remove_file(path) {
        Ok(_) => true,
        Err(_) => false,
    };
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
