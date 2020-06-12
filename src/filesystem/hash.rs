use blake2::{Blake2s, Digest};

pub fn get_image_hash(data: Vec<u8>) -> String {
    let mut hasher = Blake2s::new();
    hasher.update(&data);
    let res = hasher.finalize();

    format!("{:x}", res)
}
