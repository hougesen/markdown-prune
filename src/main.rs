mod config;
mod convert_bytes;
mod directory;
mod file;

use config::Config;
use convert_bytes::{convert_bytes, ByteSize};
use directory::{traverse_dir, DeleteResult};

fn main() -> std::io::Result<()> {
    let config = Config::new();

    if let Some(start_path) = config.path {
        if !start_path.exists() {
            println!("ERROR: Invalid path");
            return Ok(());
        }

        let result = traverse_dir(start_path, config.delete_files)?;

        print_result(result, config.delete_files, &config.unit);
    } else {
        println!("ERROR: Missing path")
    }

    Ok(())
}

fn print_result(result: DeleteResult, delete_files: bool, unit: &ByteSize) {
    let operation = if delete_files { "Deleted" } else { "Found" };

    println!(
        "{} {} files totaling {:.2}{:?}",
        operation,
        result.file_count,
        convert_bytes(result.bytes, &unit),
        &unit,
    )
}
