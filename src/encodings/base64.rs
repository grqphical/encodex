use crate::{handle_error, EXIT_INVALID_DATA};

const BASE_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(input: &[u8]) -> String {
    // Implementation based on https://keyboardsmash.dev/posts/implementing-base64-algorithm-in-rust/

    let mut output = Vec::new();
    let input_length = input.len();

    for i in (0..input_length).step_by(3) {
        let a = input.get(i).unwrap();
        let b = input.get(i + 1).unwrap_or(&0);
        let c = input.get(i + 2).unwrap_or(&0);

        let enc1 = (a >> 2) as usize;
        let enc2 = (((a & 0x3) << 4) | (b >> 4)) as usize;
        let enc3 = (((b & 0xf) << 2) | (c >> 6)) as usize;
        let enc4 = (c & 0x3f) as usize;

        output.push(BASE_CHARS[enc1]);
        output.push(BASE_CHARS[enc2]);
       
        output.push(BASE_CHARS[enc3]);
        output.push(BASE_CHARS[enc4]);
    }

    let output_len = output.len();
    let padding_len = match input_length % 3 {
        1 => 2, // Add two paddings
        2 => 1, // Add one padding
        _ => 0, // No paddings needed
    };

    for i in 0..padding_len {
        output[output_len - 1 - i] = b'=';
    }

    String::from_utf8(output).unwrap()
}

pub fn decode(input: &[u8]) -> String {
    // Implementation based on https://keyboardsmash.dev/posts/base64-implementation-in-rust-decoding/

	let mut output: Vec<u8> = Vec::new();
	for chunk in input.chunks(4) {

        if chunk.len() != 4 {
            handle_error("Invalid data provided".into(), EXIT_INVALID_DATA)
        }

    	let a = decode_char(chunk[0]);
    	let b = decode_char(chunk[1]);
    	let c = decode_char(chunk[2]);
    	let d = decode_char(chunk[3]);

    	let dec1 = ((a << 2) | (b & 0x30) >> 4) as u8;
    	let dec2 = (((b & 0x0F) << 4) | (c & 0x3C) >> 2) as u8;
    	let dec3 = (((c & 0x03) << 6) | (d)) as u8;

    	output.push(dec1);
    	output.push(dec2);
    	output.push(dec3);
	}

	String::from_utf8(output).unwrap().replace("\0", "")
}


fn decode_char(input: u8) -> u8 {
	BASE_CHARS.iter().position(|&c| c == input).unwrap_or(0) as u8
}