use crate::units::{parse_byte_type, ByteSize};

pub struct Config {
    pub path: Option<std::path::PathBuf>,
    pub delete_files: bool,
    pub unit: ByteSize,
    pub custom_bad_files: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Self {
            path: None,
            delete_files: true,
            unit: ByteSize::MB,
            custom_bad_files: Vec::new(),
        };

        let args: Vec<String> = std::env::args().collect();

        if args.len() == 2 {
            config.path = Some(std::path::PathBuf::from(args[1].clone()));
            return config;
        }

        for i in 0..args.len() {
            match args[i].as_str() {
                "--dry" | "--check" => config.delete_files = false,
                "--path" => {
                    if i + 1 < args.len() && !(&args[i + 1].starts_with("--")) {
                        let path = std::path::PathBuf::from(args[i + 1].clone());
                        config.path = Some(path);
                    } else {
                        eprintln!("ERROR: --path is missing an input or is invalid");
                        std::process::exit(22);
                    }
                }
                "--unit" => {
                    if i + 1 < args.len() && !(&args[i + 1].starts_with("--")) {
                        config.unit = parse_byte_type(&args[i + 1]);
                    } else {
                        eprintln!("WARNING: --unit is missing an input or is invalid");
                        std::process::exit(22);
                    }
                }
                "--file" | "--file-name" | "--ext" => {
                    if i + 1 < args.len() && !(&args[i + 1].starts_with("--")) {
                        config.custom_bad_files.push(args[i + 1].to_string());
                    } else {
                        eprintln!("WARNING: --file is missing an input or is invalid");
                        std::process::exit(22);
                    }
                }
                _ => continue,
            }
        }

        config
    }
}
