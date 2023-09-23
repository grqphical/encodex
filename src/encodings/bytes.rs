use crate::{handle_error, EXIT_INVALID_DATA};

pub fn encode(data: &[u8], formatted: bool) -> String {
    let mut output = String::new();
    for byte in data {
        if formatted {
            output.push_str(&format!("{} ", byte))
        } else {
            output.push_str(&format!("{}", byte))
        }
        
    }

    output = output.trim_end().into();

    output
}

pub fn decode(data: String) -> String {
    let bytes: Vec<u8> = data.split(" ").map(|x| {
        let result = x.parse::<u8>();
        match result {
            Ok(value) => value,
            Err(_) => {
                handle_error("Invalid byte data".into(), EXIT_INVALID_DATA);
                0
            }
        }
    }).collect();
    let mut output = String::new();

    for byte in bytes {
        output.push(byte as char)
    }
    output
}