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

use std::{fs::File, io::prelude::*, io::BufReader};
fn main() {
    // Parse lines first

    let file: File = File::open("data.txt").expect("No file found.");
    let mut reader = BufReader::new(file);
    let lines: Vec<Vec<u8>> = reader
        .lines()
        .map(|x| {
            return parse_hexstring(String::from(x.unwrap()));
        })
        .collect();
    println!("File succesfully read.");
    // If a line is broken into groups of 16 bytes, how many times do duplicates occur?
    let mut duplicate_occured: Vec<Vec<u8>> = Vec::new();

    for line in lines {
        let mut duplicates = 0;
        let mut seen: Vec<Vec<u8>> = Vec::new();
        for group in line.chunks(16) {
            match seen.contains(&group.to_vec()) {
                false => {
                    seen.push(group.to_vec());
                }
                true => {
                    duplicates += 1;
                }
            }
        }
        if duplicates != 0 {
            duplicate_occured.push(line);
        }
    }

    println!(
        "Duplicates occured for {} string(s):",
        duplicate_occured.len()
    );
    for i in duplicate_occured {
        println!("{}", bytes_to_hex(i))
    }
    println!("So it's the most likely candidate to be encrypted with ECB")
}
