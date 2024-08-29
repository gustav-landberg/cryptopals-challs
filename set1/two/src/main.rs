// By Gustav Landberg, 2024

use std::str::FromStr;

// This is a message that has been XORed by a one byte key and then encoded in hex.
const INPUT: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

// The goal is to recover the original message.
// TO that end, a scoring system is needed to identify the decrypted message programatically.. The description suggest we use freuqency
// of letters as a scoring system

fn letter_frequency(letter: char) -> i32 {
    // let's see where ranking vowels get us
    match letter.to_ascii_lowercase() {
        // intially just typed in just higher scores for vowels and the english alphabet,
        // however it is now clear that i was accidentally demoting atmostrophes and spaces, which
        // also occur in english plaintext
        'e' => 13,
        'a' | 'o' => 8,
        's' => 12,
        'i' => 7,
        'u' => 3,
        'a'..='z' | ' ' | '\'' => 1,
        '0'..='9' => 0,
        // non-ascii letters
        _ => -10,
    }
}

fn bytes_score(bytes: Vec<u8>) -> i32 {
    bytes.into_iter().map(|x| letter_frequency(x as char)).sum()
}

fn single_byte_xor(bytes: Vec<u8>, key: u8) -> Vec<u8> {
    bytes.into_iter().map(|x| x ^ key).collect()
}

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
    output
}
// to hexstring
fn bytes_to_hex(bytes: Vec<u8>) -> String {
    let symbols = b"0123456789abcdef";
    let mut output = String::new();
    for i in bytes {
        let upper = symbols[((i & 0xf0) >> 4) as usize] as char;
        let lower = symbols[(i & 0x0f) as usize] as char;

        output.push(upper);
        output.push(lower);
    }
    output
}

fn main() {
    let input_bytes: Vec<u8> = parse_hexstring(String::from_str(INPUT).unwrap());
    let (mut highest_score, mut highest_key): (i32, u8) = (i32::MIN, 0);
    // linear search for the right key
    for key in 0..=255 {
        /*println!(
            "Trying {key}: {}",
            String::from_utf8(single_byte_xor(input_bytes.clone(), key)).unwrap()
        );*/
        let score = bytes_score(single_byte_xor(input_bytes.clone(), key));
        if (score > highest_score) {
            (highest_score, highest_key) = (score, key);
        }
    }

    let message_bytes = (single_byte_xor(input_bytes.clone(), highest_key));

    println!(
        "Most likely, the message is the one with score {} and key {}: ",
        highest_score, highest_key
    );
    println!("{}", String::from_utf8(message_bytes).unwrap());
}
