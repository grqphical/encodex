mod encodings;

use clap::{Parser, ValueEnum};
use encodings::{base64, binary, bytes, hex};
use std::{path::PathBuf, str};

// Tests
#[cfg(test)]
mod tests {
    use crate::encodings;
    #[test]
    fn base64_encode_test() {
        let test_data = "Hello World";
        let expected = "SGVsbG8gV29ybGQ=";

        assert_eq!(encodings::base64::encode(test_data.as_bytes()), expected)
    }
    #[test]
    fn base64_decode_test() {
        let test_data = "SGVsbG8gV29ybGQ=";
        let expected = "Hello World";

        assert_eq!(encodings::base64::decode(test_data.as_bytes()), expected)
    }
    #[test]
    fn binary_encode_test() {
        let test_data = "Hello World";
        let expected = "1001000 1100101 1101100 1101100 1101111 100000 1010111 1101111 1110010 1101100 1100100";

        assert_eq!(encodings::binary::encode(test_data.as_bytes(), true), expected)
    }

    #[test]
    fn binary_decode_test() {
        let test_data = "1001000 1100101 1101100 1101100 1101111 100000 1010111 1101111 1110010 1101100 1100100";
        let expected = "Hello World";

        assert_eq!(encodings::binary::decode(test_data.into()), expected)
    }

    #[test]
    fn bytes_decode_test() {
        let test_data = "72 101 108 108 111 32 87 111 114 108 100";
        let expected = "Hello World";

        assert_eq!(encodings::bytes::decode(test_data.into()), expected)
    }

    #[test]
    fn bytes_encode_test() {
        let test_data = "Hello World";
        let expected = "72 101 108 108 111 32 87 111 114 108 100";

        assert_eq!(encodings::bytes::encode(test_data.as_bytes(), true), expected)
    }

    #[test]
    fn hex_decode_test() {
        let test_data = "48656c6c6f20576f726c64";
        let expected = "Hello World";

        let decoding_result = encodings::hex::decode(test_data).unwrap();
        let s = std::str::from_utf8(&decoding_result).unwrap();

        assert_eq!(s, expected)
    }

    #[test]
    fn hex_encode_test() {
        let test_data = "Hello World";
        let expected = "48656c6c6f20576f726c64";

        assert_eq!(encodings::hex::encode(test_data.as_bytes()), expected)
    }
}

// EXIT CODES
const EXIT_FILE_NOT_FOUND: i32 = 2;
const EXIT_INVALID_UTF8: i32 = 3;
const EXIT_NO_DATA_PROVIDED: i32 = 4;
const EXIT_INVALID_DATA: i32 = 5;

/// Function to gracefully handle errors by printing a message and exiting
///
/// # Arguments
///
/// * `err` - The error message
///
/// * `code` - The error code to exit with
///
/// # Examples
///
/// ```
/// handle_error("Invalid data".into(), 1);
/// ```
pub fn handle_error(err: String, code: i32) {
    println!("\x1b[1;31mERROR: {}\x1b[0;0m", err);
    std::process::exit(code);
}

/// All avalible encoding formats
#[derive(ValueEnum, Clone, Debug)]
enum Encodings {
    Base64,
    Hex,
    Binary,
    Bytes,
}

/// Structure of how command line args should be parsed
#[derive(Parser, Debug)]
#[command(version, author = "grqphical07", about = "A simple tool to encode strings in different formats", long_about = None)]
struct Cli {
    /// Optional text to encode
    text: Option<String>,

    /// Optional file to encode
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Whether or not you wish to encode or decode
    #[arg(short, long)]
    decode: bool,

    /// What encoding format do you wish to use
    #[arg(short, long)]
    encoding: Encodings,

    /// Whether or not the value should be formatted
    #[arg(long)]
    formatted: bool,
}

fn main() {
    // Parse the cli args
    let cli = Cli::parse();

    let mut text = String::new();

    // Check if user wants to read from a file and if so read from it
    // If no file is specified use the text the user entered into the application
    match cli.file {
        Some(val) => {
            let file_result = std::fs::read_to_string(val);
            match file_result {
                Ok(value) => text = value,
                Err(err) => handle_error(err.to_string(), EXIT_FILE_NOT_FOUND),
            }
        }
        None => match cli.text {
            Some(value) => text = value,
            None => handle_error(
                "No data provided. Use --help for options".into(),
                EXIT_NO_DATA_PROVIDED,
            ),
        },
    }

    // Encode/Decode the data based on which format the user wants to use
    match cli.encoding {
        Encodings::Base64 => {
            if cli.decode {
                println!("{}", base64::decode(text.as_bytes()))
            } else {
                println!("{}", base64::encode(text.as_bytes()))
            }
        }
        Encodings::Hex => {
            if cli.decode {
                let decoding_result = hex::decode(&text);
                let mut decoded_bytes: Vec<u8> = Vec::new();

                match decoding_result {
                    Ok(value) => decoded_bytes = value,
                    Err(_) => handle_error("Invalid Hex Data".into(), EXIT_INVALID_DATA),
                }

                let s = match str::from_utf8(&decoded_bytes) {
                    Ok(v) => v,
                    Err(_) => {
                        handle_error("Invalid Text Sequence".into(), EXIT_INVALID_UTF8);
                        ""
                    }
                };
                println!("{}", s)
            } else {
                println!("{}", hex::encode(text.as_bytes()))
            }
        }
        Encodings::Bytes => {
            if cli.decode {
                println!("{}", bytes::decode(text))
            } else {
                println!("{}", bytes::encode(text.as_bytes(), cli.formatted))
            }
        }
        Encodings::Binary => {
            if cli.decode {
                println!("{}", binary::decode(text))
            } else {
                println!("{}", binary::encode(text.as_bytes(), cli.formatted))
            }
        }
    }
}
