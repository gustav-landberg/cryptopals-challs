fn decode_b64_char(symbol: char) -> Option<u8> {
    match symbol {
        'A'..='Z' => Some(symbol as u8 - ('A' as u8)),
        'a'..='z' => Some(symbol as u8 - ('a' as u8) + 26),
        '0'..='9' => Some(symbol as u8 - ('0' as u8) + 52),
        '+' => Some(62),
        '/' => Some(63),
        _ => None,
    }
}

fn b64_to_bytes(b64_bytes: Vec<u8>) -> Vec<u8> {
    b64_bytes
        .chunks(4)
        .flat_map(|x| b64_group_to_bytes(&[x[0], x[1], x[2], x[3]]))
        .collect()
}

fn b64_group_to_bytes(encoded_bytes: &[u8; 4]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    let values: Vec<Option<u8>> = encoded_bytes
        .into_iter()
        .map(|x| decode_b64_char(x.clone() as char))
        .collect();

    let (mut b0, mut b1, mut b2): (u8, u8, u8) = (0, 0, 0);

    let b0c = values[0].is_some();
    let b1c = b0c && values[1].is_some() && values[2].is_some();
    let b2c = b0c && b1c && values[3].is_some();
    // this is a bit redundant
    if values[0].is_some() {
        b0 = b0 | (values[0].unwrap_or(0) << 2);
        if values[1].is_some() {
            b0 = b0 | ((values[1].unwrap_or(0) & 0b00110000) >> 4);
            if values[2].is_some() {
                b1 = b1 | (values[1].unwrap_or(0) << 4);
                b1 = b1 | ((values[2].unwrap_or(0) & 0b00111100) >> 2);
                if values[3].is_some() {
                    b2 = b2 | (values[2].unwrap_or(0) << 6);
                    b2 = b2 | (values[3].unwrap_or(0));
                }
            }
        }
    }
    if b0c {
        output.push(b0);
    }
    if b1c {
        output.push(b1);
    }
    if b2c {
        output.push(b2);
    }
    output
}

// AES is a complicated algorithm to implement. I use the openssl bindings instead.
fn decrypt_aes_128_ecb(ciphertext: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    use openssl::symm::*;
    let cipher = Cipher::aes_128_ecb();
    decrypt(cipher, key, None, ciphertext).unwrap()
}

fn main() {
    println!("Hello, world!");
    print!("Reading file... ");
    // 16 bytes or 128 bits
    let key = b"YELLOW SUBMARINE".to_vec();
    match std::fs::read_to_string("data.txt") {
        Ok(filestring) => {
            println!("OK!");
            println!("Processing indata...");
            let filedata: Vec<u8> = b64_to_bytes(
                filestring
                    .chars()
                    .filter(|x| *x != '\n')
                    .map(|x| x as u8)
                    .collect(),
            );
            let plaintext_bytes = decrypt_aes_128_ecb(&filedata, &key);
            let plaintext = String::from_utf8_lossy(&plaintext_bytes);
            println!("{}", plaintext);
        }
        Err(_) => {
            println!("Something went wrong.");
        }
    }
}
