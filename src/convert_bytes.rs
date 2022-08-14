#[derive(Debug)]
pub enum ByteSize {
    B,
    KB,
    MB,
    GB,
    TB,
}

/// Defaults to mb
pub fn parse_byte_type(arg: &str) -> ByteSize {
    match arg.trim().to_lowercase().as_str() {
        "b" => ByteSize::B,
        "kb" => ByteSize::KB,
        "mb" => ByteSize::MB,
        "gb" => ByteSize::GB,
        "tb" => ByteSize::TB,
        _ => ByteSize::MB,
    }
}

pub fn convert_bytes(bytes: u64, format: &ByteSize) -> f64 {
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
