mod config;
mod directory;
mod file;
mod units;

use config::Config;
use directory::{traverse_dir, DeleteResult};
use units::{convert_bytes, ByteSize};

fn main() -> std::io::Result<()> {
    let config = Config::new();

    if let Some(start_path) = config.path {
        if !start_path.exists() {
            eprintln!("ERROR: Path {:#?} does not exist", start_path);
            std::process::exit(2);
        }

        let result = traverse_dir(start_path, config.delete_files, &config.custom_bad_files)?;

        print_result(result, config.delete_files, &config.unit);
    } else {
        eprintln!("ERROR: Missing path")
    }

    Ok(())
}

fn print_result(result: DeleteResult, delete_files: bool, unit: &ByteSize) {
    let operation = if delete_files { "Deleted" } else { "Found" };

    println!(
        "{} {} files totaling {:.2}{:?}",
        operation,
        result.file_count,
        convert_bytes(result.bytes, unit),
        &unit,
    )
}
