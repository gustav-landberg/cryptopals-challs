use std::{fs, str::FromStr};

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

fn bytes_count_duplicates(bytes: Vec<u8>) -> i32 {
    let mut found: Vec<u8> = Vec::new();
    let mut score: i32 = 0;
    for i in bytes {
        if found.contains(&i) {
            score += 1;
        } else {
            found.push(i);
        }
    }
    score
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
    print!("Reading file... ");
    match fs::read_to_string("4.txt") {
        Ok(filestr) => {
            println!("OK!");
            println!("Processing indata...");
            let byte_lines: Vec<Vec<u8>> = filestr
                .split("\n")
                .map(|x| parse_hexstring(String::from_str(x).unwrap()))
                .collect();
            // we could probably recognize xor encryption by ranking the strings based on
            // most duplicates:
            let highest = byte_lines
                .into_iter()
                .max_by_key(|x| bytes_count_duplicates(x.to_vec()));
            println!("{}", bytes_to_hex(highest.clone().unwrap()));
            println!("Attempting to brute-force suspected ciphertext");
            let keys: std::ops::RangeInclusive<u8> = 0..=255;
            let chosen_key = keys
                .max_by_key(|x| {
                    let copy = x.clone();
                    bytes_score(single_byte_xor(highest.clone().unwrap(), copy))
                })
                .unwrap();
            println!(
                "{}",
                String::from_utf8(single_byte_xor(highest.clone().unwrap(), chosen_key)).unwrap()
            )
        }
        Err(_) => {
            println!("FAIL!");
            println!("Could not read file, are you sure you are in the correct working directory?");
        }
    }
}
