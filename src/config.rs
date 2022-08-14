use crate::convert_bytes::{parse_byte_type, ByteSize};

pub struct Config {
    pub path: Option<std::path::PathBuf>,
    pub delete_files: bool,
    pub unit: ByteSize,
}

impl Config {
    pub fn new() -> Config {
        let mut config = Config {
            path: None,
            delete_files: true,
            unit: ByteSize::MB,
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
                    if i + 1 < args.len() {
                        let path = std::path::PathBuf::from(args[i + 1].clone());
                        config.path = Some(path);
                    } else {
                        println!("ERROR: --path is missing an input");
                    }
                }
                "--unit" => {
                    if i + 1 < args.len() {
                        config.unit = parse_byte_type(&args[i + 1]);
                    } else {
                        println!("WARNING: --unit is missing an input");
                    }
                }
                _ => continue,
            }
        }

        config
    }
}
