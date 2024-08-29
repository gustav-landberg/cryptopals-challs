use std::str::FromStr;

const INPUT1: &str = "1c0111001f010100061a024b53535009181c";
const INPUT2: &str = "686974207468652062756c6c277320657965"; // key
const OUTPUT: &str = "746865206b696420646f6e277420706c6179";

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

fn fixed_xor_bytes(bytes1: Vec<u8>, bytes2: Vec<u8>) -> Vec<u8> {
    let mut i1 = bytes1.iter();
    let mut i2 = bytes2.iter();
    let mut output: Vec<u8> = Vec::new();
    loop {
        let pop1 = i1.next();
        let pop2 = i2.next();

        if pop1.is_some() && pop2.is_some() {
            output.push(pop1.unwrap() ^ pop2.unwrap());
        } else {
            break;
        }
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
    println!("Hello, world!");

    let actual_output = bytes_to_hex(fixed_xor_bytes(
        parse_hexstring(String::from_str(INPUT1).unwrap()),
        parse_hexstring(String::from_str(INPUT2).unwrap()),
    ));
    println!("Inputs: {} {}", INPUT1, INPUT2);
    println!("Outputs: {}", actual_output.as_str());
    assert_eq!(actual_output, String::from_str(OUTPUT).unwrap());
    println!("Checks passed");
}
