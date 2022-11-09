use sha2::{Digest, Sha256, Sha512};

pub fn sha256(msg: &Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(msg);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn sha512(msg: &Vec<u8>) -> String {
    let mut hasher = Sha512::new();
    hasher.update(msg);
    let result = hasher.finalize();
    hex::encode(result)
}
