use crate::{handle_error, EXIT_INVALID_DATA};

pub fn encode(data: &[u8]) -> String {
    let mut output = String::new();

    for byte in data {
        output.push_str(&format!("{:b} ", byte))
    }

    output = output.trim_end().into();

    output
}

pub fn decode(data: String) -> String {
    let chunks = data.split(" ");
    let mut output = String::new();

    for chunk in chunks {
        // Convert the string into a u8
        let conversion_result = u8::from_str_radix(chunk, 2);
        let mut integer_value: u8 = 0;

        match conversion_result {
            Ok(val) => integer_value = val,
            Err(_) => handle_error("Invalid Binary".into(), EXIT_INVALID_DATA)
        }

        // Add it to the result string as a character
        let char = integer_value as char;

        output.push(char)
    }

    output
}