use std::{ffi::OsStr, path::Path};

fn main() {
    // TODO: accept args instead

    let start_path = Path::new("dummy-files");

    traverse_dir(start_path);
}

fn traverse_dir(path: &Path) -> std::io::Result<()> {
    for entry in std::fs::read_dir(&path)? {
        println!("{:?} entry {:?}", path, entry);

        let entry_path = entry?.path();

        if entry_path.is_dir() {
            traverse_dir(entry_path.as_path())?;
        } else if entry_path.is_file() {
            let file_extension = entry_path.extension();
            let bad_file = check_if_bad_file(file_extension);

            if bad_file {
                delete_file(entry_path.as_path());
            }
        }
    }

    Ok(())
}

fn check_if_bad_file(file_extension: Option<&OsStr>) -> bool {
    return match file_extension {
        Some(ext) => {
            if ext == "md" {
                return true;
            }

            return false;
        }
        None => true,
    };
}

fn delete_file(path: &Path) -> bool {
    println!("DELETE FILE {:?}", path);

    return match std::fs::remove_file(path) {
        Ok(_) => true,
        Err(_) => false,
    };
}
