// pkcs7

fn apply_pkcs7_padding(bytes: &Vec<u8>, block_size: usize) -> Vec<u8> {
    let delta = block_size - bytes.len();
    let mut output = bytes.clone();
    for _ in 0..delta {
        output.push(delta as u8)
    }
    output
}

fn main() {
    let bytes = b"YELLOW SUBMARINE";
    assert_eq!(
        b"YELLOW SUBMARINE\x04\x04\x04\x04".to_vec(),
        apply_pkcs7_padding(&bytes.to_vec(), 20)
    );
    println!("Hello, world!");
}
