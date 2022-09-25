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
mod units {
    mod test_parse_byte_type {
        use crate::units::{parse_byte_type, ByteSize};

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

    mod test_convert_bytes {
        use crate::units::{convert_bytes, ByteSize};

        #[test]
        fn all_convert_bytes() {
            let tb = convert_bytes(1000000000000, &ByteSize::TB);
            assert!(tb == 1.0);

            let gb = convert_bytes(1000000000000, &ByteSize::GB);
            assert!(gb == 1000.0);

            let mb = convert_bytes(1000000000000, &ByteSize::MB);
            assert!(mb == 1000000.0);

            let kb = convert_bytes(1000000000000, &ByteSize::KB);
            assert!(kb == 1000000000.0);

            let b = convert_bytes(1000000000000, &ByteSize::B);
            assert!(b == 1000000000000.0);
        }

        #[test]
        fn b_convert_bytes() {
            assert!(convert_bytes(1, &ByteSize::B) == 1.0);
            assert!(convert_bytes(10, &ByteSize::B) == 10.0);
            assert!(convert_bytes(100, &ByteSize::B) == 100.0);
            assert!(convert_bytes(1000, &ByteSize::B) == 1000.0);
        }

        #[test]
        fn kb_convert_bytes() {
            assert!(convert_bytes(1000, &ByteSize::KB) == 1.0);
            assert!(convert_bytes(10000, &ByteSize::KB) == 10.0);
            assert!(convert_bytes(100000, &ByteSize::KB) == 100.0);
            assert!(convert_bytes(1000000, &ByteSize::KB) == 1000.0);
        }

        #[test]
        fn mb_convert_bytes() {
            assert!(convert_bytes(1000000, &ByteSize::MB) == 1.0);
            assert!(convert_bytes(10000000, &ByteSize::MB) == 10.0);
            assert!(convert_bytes(100000000, &ByteSize::MB) == 100.0);
            assert!(convert_bytes(1000000000, &ByteSize::MB) == 1000.0);
        }

        #[test]
        fn gb_convert_bytes() {
            assert!(convert_bytes(1000000000, &ByteSize::GB) == 1.0);
            assert!(convert_bytes(10000000000, &ByteSize::GB) == 10.0);
            assert!(convert_bytes(100000000000, &ByteSize::GB) == 100.0);
            assert!(convert_bytes(1000000000000, &ByteSize::GB) == 1000.0);
        }

        #[test]
        fn tb_convert_bytes() {
            assert!(convert_bytes(1000000000000, &ByteSize::TB) == 1.0);
        }
    }
}
