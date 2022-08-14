mod config;
mod convert_bytes;
mod directory;
mod file;

use config::Config;
use convert_bytes::{convert_bytes, ByteSize};
use directory::traverse_dir;

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
