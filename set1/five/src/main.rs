use std::collections::HashMap;

/*
* Returns Ok(distance) if the two vectors have equal length.
* Err(()) otherwise.
* */
fn hamming_distance_bytes(bytes1: Vec<u8>, bytes2: Vec<u8>) -> Result<u32, ()> {
    if (bytes1.len() != bytes2.len()) {
        return Err(());
    } else {
        let indices: std::ops::Range<usize> = 0..bytes1.len();
        let distance = indices
            .into_iter()
            .map(|x| (bytes1[x] ^ bytes2[x]).count_ones())
            .sum();
        return Ok(distance);
    }
}

fn find_cyclic_xor_keysize(ciphertext: Vec<u8>, guess_upper_bound: usize) -> Vec<u32> {
    let mut distances: Vec<(f32, u32)> = Vec::new();
    // Find distances in first two blocks of ciphertext
    for i in 2..=guess_upper_bound {
        let chunks: Vec<&[u8]> = ciphertext.chunks(i).collect();
        let sl1 = chunks.get(0).unwrap().to_vec();
        let sl2 = chunks.get(1).unwrap().to_vec();
        distances.push((
            hamming_distance_bytes(sl1, sl2).unwrap() as f32 / (i as f32),
            i as u32,
        ));
    }
    distances.sort_by(|(a, _), (b, _)| a.partial_cmp(&b).unwrap());
    println!("{:?}", distances);
    return distances.into_iter().map(|x| x.1).collect();
}
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

fn find_repeating_xor_key(ciphertext: Vec<u8>, keysize: usize) -> Vec<u8> {
    //println!(
    //     "Searching for key. \n Ciphertext: {:?}  \n keysize: {:?}",
    //     ciphertext, keysize
    // );
    let mut index_blocks: Vec<Vec<u8>> = Vec::new();
    for _ in 0..keysize {
        index_blocks.push(Vec::new());
    }
    for i in ciphertext.chunks(keysize) {
        for j in 0..keysize {
            match i.get(j) {
                Some(byte) => index_blocks[j].push(byte.clone()),
                None => (),
            }
        }
    }
    // println!("blocks: {:?}", index_blocks);
    let mut result_key: Vec<u8> = Vec::new();
    for i in index_blocks {
        let (mut highest_score, mut highest_key): (i32, u8) = (i32::MIN, 0);
        // linear search for the right key
        for key in 0..=255 {
            let score = bytes_score(single_byte_xor(i.clone(), key as u8));

            if (score > highest_score) {
                //println!("score: {} key: {}", score, key);
                (highest_score, highest_key) = (score, key);
            }
        }
        // println!("Highest key: 0x{:02x}", highest_key);
        result_key.push(highest_key);
    }
    result_key
}

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

fn repeating_xor(msg: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let mut key_iter = key.into_iter().cycle();
    let mut output: Vec<u8> = Vec::new();
    for i in msg {
        output.push(i ^ key_iter.next().unwrap());
    }
    output
}

fn main() {
    assert_eq!(
        hamming_distance_bytes(b"this is a test".to_vec(), b"wokka wokka!!!".to_vec()),
        Ok(37)
    );
    print!("Reading file... ");
    match std::fs::read_to_string("6.txt") {
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
            let keysizes = find_cyclic_xor_keysize(filedata.clone(), 40);
            for i in keysizes {
                let result = find_repeating_xor_key(filedata.clone(), i.clone() as usize);
                if bytes_score(repeating_xor(filedata.clone(), result.clone())) > 10 {
                    println!("The key is probably: {}", String::from_utf8_lossy(&result));
                    println!(
                        "Decrypted message: {}",
                        String::from_utf8_lossy(&repeating_xor(filedata.clone(), result))
                    );
                }
            }
        }
        Err(_) => {
            println!("FAIL!");
            println!("Could not read file, are you sure you are in the correct working directory?");
        }
    }
}
