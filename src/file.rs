const BAD_EXTENSIONS: [&str; 3] = ["markdown", "md", "mkd"];

pub fn check_if_bad_file_ext(file_extension: Option<&std::ffi::OsStr>) -> bool {
    match file_extension {
        Some(ext) => {
            // No need to check BAD_EXTENSIONS if the ext length is less than the length of "md"
            if ext.len() < 2 {
                return false;
            }

            if let Some(ext_str) = ext.to_str() {
                return BAD_EXTENSIONS.binary_search(&ext_str).is_ok();
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

#[cfg(test)]
mod test_file {
    mod test_check_if_bad_file_ext {
        use crate::file::{check_if_bad_file_ext, BAD_EXTENSIONS};

        #[test]
        fn true_if_in_bad_extensions() {
            BAD_EXTENSIONS
                .iter()
                .for_each(|ext| assert!(check_if_bad_file_ext(Some(std::ffi::OsStr::new(ext)))))
        }

        #[test]
        fn false_if_not_in_bad_extensions() {
            assert!(check_if_bad_file_ext(Some(std::ffi::OsStr::new("not-a-bad-ext"))) == false);

            assert!(
                check_if_bad_file_ext(Some(std::ffi::OsStr::new("not-a bad-ext-either"))) == false
            );
        }
    }

    mod test_check_if_bad_file {
        use crate::file::check_if_bad_file;

        #[test]
        fn test_check_if_bad_file() {
            let bad_files = vec![
                ".DS_Store".to_string(),
                ".md".to_string(),
                ".d.ts".to_string(),
            ];

            assert!(check_if_bad_file(std::path::Path::new("./README.md"), &bad_files) == true);
            assert!(check_if_bad_file(std::path::Path::new("./DS_Store.md"), &bad_files) == true);
            assert!(check_if_bad_file(std::path::Path::new("./README.d.ts"), &bad_files) == true);
            assert!(check_if_bad_file(std::path::Path::new("./dummy.md"), &bad_files) == true);

            assert!(check_if_bad_file(std::path::Path::new("./dummy.ts"), &bad_files) == false);
            assert!(check_if_bad_file(std::path::Path::new("./dummy.cpp"), &bad_files) == false);
            assert!(check_if_bad_file(std::path::Path::new("./dummy.rs"), &bad_files) == false);
            assert!(check_if_bad_file(std::path::Path::new("./dummy.nim"), &bad_files) == false);
        }
    }
}
