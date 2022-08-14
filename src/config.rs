pub struct Config {
    pub path: Option<std::path::PathBuf>,
    pub delete: bool,
}

impl Config {
    pub fn new() -> Config {
        let mut config = Config {
            path: None,
            delete: true,
        };

        let args: Vec<String> = std::env::args().collect();

        if args.len() == 2 {
            config.path = Some(std::path::PathBuf::from(args[1].clone()));
            return config;
        }

        for i in 0..args.len() {
            match args[i].as_str() {
                "--dry" | "--check" => config.delete = false,
                "--path" => {
                    if i + 1 < args.len() {
                        let path = std::path::PathBuf::from(args[i + 1].clone());
                        config.path = Some(path);
                    }
                }
                _ => continue,
            }
        }

        config
    }
}
