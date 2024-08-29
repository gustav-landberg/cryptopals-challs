// convert hex to base 64
// String
// 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
// should produce
// SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t

const INPUT : &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const OUTPUT: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

fn hex_char_to_number(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - '0' as u8),
        'A'..='F' => Some(c as u8 - 'A' as u8 + 10),
        'a'..='f' => Some(c as u8 - 'a' as u8 + 10),
        _ => None,
    }
}

fn parse_hexstring(input: String) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    for nibbles in chars.chunks(2) {
        output.push(
            (hex_char_to_number(nibbles[0]).unwrap() << 4)
                | hex_char_to_number(nibbles[1]).unwrap(),
        );
    }
    println!("{:?}", output);
    output
}

fn bytes_to_b64(input: Vec<u8>) -> String {
    let mut output = String::new();
    // Base64 encodes per 6 bits in groups of 24
    // We are working with bytes which have 8 bits.

    for bytes in input.chunks(3) {
        let b1 = bytes.get(1);
        let b2 = bytes.get(2);

        // println!("{:b}, {:b}, {:b}", bytes[0], bytes[1], bytes[2]);
        let (o0, o1, o2, o3): (u8, Option<u8>, Option<u8>, Option<u8>);

        o0 = (bytes[0] & 0xfc) >> 2;
        o1 = if b1.is_some() {
            Some(((bytes[0] & 0x3) << 4) | ((b1.unwrap() & 0xf0) >> 4))
        } else {
            None
        };
        o2 = if b1.is_some() && b2.is_some() {
            Some(((b1.unwrap() & 0x0f) << 2) | ((b2.unwrap() & 0xc0) >> 6))
        } else {
            None
        };
        o3 = if b2.is_some() {
            Some((b2.unwrap() & 0x3f))
        } else {
            None
        };
        output.push(b64_symbol(o0).unwrap());
        match o1 {
            Some(num) => output.push(b64_symbol(num).unwrap()),
            None => output.push('='),
        }
        match o2 {
            Some(num) => output.push(b64_symbol(num).unwrap()),
            None => output.push('='),
        }
        match o3 {
            Some(num) => output.push(b64_symbol(num).unwrap()),
            None => output.push('='),
        }
    }
    output
}

fn b64_symbol(numb: u8) -> Option<char> {
    println!("encoding: {}", numb);
    let b64_symbols = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    if numb > 63 {
        None
    } else {
        Some(b64_symbols[numb as usize] as char)
    }
}

fn main() {
    let result: String = bytes_to_b64(parse_hexstring(INPUT.to_string()));
    assert_eq!(result, OUTPUT.to_string());
    println!("Test passed");
    println!("Initializing echo server... enter a valid hexstring");
    loop {
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        println!(
            "{}",
            bytes_to_b64(parse_hexstring(input.trim().to_string()))
        );
    }
}
