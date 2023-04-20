use rand::{self, Rng, distributions::{Alphanumeric}};
use sha256;


pub fn gen_rand_str(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn gen_token(id: String) -> String {
    let id = sha256::digest(id);
    format!("{id}.{}", gen_rand_str(16))
}