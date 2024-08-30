// By Gustav Landberg, 2024

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

fn repeating_xor(msg: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let mut key_iter = key.into_iter().cycle();
    let mut output: Vec<u8> = Vec::new();
    for i in msg {
        output.push(i ^ key_iter.next().unwrap());
    }
    output
}

fn main() {
    let mut args = std::env::args().skip(1);
    if args.len() != 2 {
        println!("usage: <hex_message> <hex_key>");
    } else {
        let msg: String = args.next().unwrap();
        let key: String = args.next().unwrap();
        println!(
            "{}",
            bytes_to_hex(repeating_xor(
                msg.clone().into_bytes(),
                key.clone().into_bytes()
            ))
        );
    }
}
