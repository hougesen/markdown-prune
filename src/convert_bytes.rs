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

#[cfg(test)]
mod test_parse_byte_type {
    use crate::convert_bytes::{parse_byte_type, ByteSize};

    #[test]
    fn none_parse_byte_type() {
        assert!(matches!(parse_byte_type(""), ByteSize::MB));
    }

    #[test]
    fn b_parse_byte_type() {
        assert!(matches!(parse_byte_type("b"), ByteSize::B));
        assert!(matches!(parse_byte_type("  b   "), ByteSize::B));
        assert!(matches!(parse_byte_type("b   "), ByteSize::B));
        assert!(matches!(parse_byte_type("  b"), ByteSize::B));
        assert!(matches!(parse_byte_type("B"), ByteSize::B));
        assert!(matches!(parse_byte_type("  B   "), ByteSize::B));
        assert!(matches!(parse_byte_type("B   "), ByteSize::B));
        assert!(matches!(parse_byte_type("  B"), ByteSize::B));
    }

    #[test]
    fn kb_parse_byte_type() {
        assert!(matches!(parse_byte_type("kb"), ByteSize::KB));
        assert!(matches!(parse_byte_type("  kb   "), ByteSize::KB));
        assert!(matches!(parse_byte_type("kb   "), ByteSize::KB));
        assert!(matches!(parse_byte_type("  kb"), ByteSize::KB));
        assert!(matches!(parse_byte_type("KB"), ByteSize::KB));
        assert!(matches!(parse_byte_type("  KB   "), ByteSize::KB));
        assert!(matches!(parse_byte_type("KB   "), ByteSize::KB));
        assert!(matches!(parse_byte_type("  kB"), ByteSize::KB));
    }

    #[test]
    fn mb_parse_byte_type() {
        assert!(matches!(parse_byte_type("mb"), ByteSize::MB));
        assert!(matches!(parse_byte_type("  mb   "), ByteSize::MB));
        assert!(matches!(parse_byte_type("mb   "), ByteSize::MB));
        assert!(matches!(parse_byte_type("  mb"), ByteSize::MB));
        assert!(matches!(parse_byte_type("MB"), ByteSize::MB));
        assert!(matches!(parse_byte_type("  MB   "), ByteSize::MB));
        assert!(matches!(parse_byte_type("MB   "), ByteSize::MB));
        assert!(matches!(parse_byte_type("  MB"), ByteSize::MB));
    }

    #[test]
    fn gb_parse_byte_type() {
        assert!(matches!(parse_byte_type("gb"), ByteSize::GB));
        assert!(matches!(parse_byte_type("  gb   "), ByteSize::GB));
        assert!(matches!(parse_byte_type("gb   "), ByteSize::GB));
        assert!(matches!(parse_byte_type("  gb"), ByteSize::GB));
        assert!(matches!(parse_byte_type("gB"), ByteSize::GB));
        assert!(matches!(parse_byte_type("  GB   "), ByteSize::GB));
        assert!(matches!(parse_byte_type("gB   "), ByteSize::GB));
        assert!(matches!(parse_byte_type("  GB"), ByteSize::GB));
    }

    #[test]
    fn tb_parse_byte_type() {
        assert!(matches!(parse_byte_type("tb"), ByteSize::TB));
        assert!(matches!(parse_byte_type("  tb   "), ByteSize::TB));
        assert!(matches!(parse_byte_type("tb   "), ByteSize::TB));
        assert!(matches!(parse_byte_type("  tb"), ByteSize::TB));
        assert!(matches!(parse_byte_type("tB"), ByteSize::TB));
        assert!(matches!(parse_byte_type("  tB   "), ByteSize::TB));
        assert!(matches!(parse_byte_type("tB   "), ByteSize::TB));
        assert!(matches!(parse_byte_type("  TB"), ByteSize::TB));
    }
}
