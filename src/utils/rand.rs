use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;

pub fn generate_random_string(length: i32) -> String {
    let char_pool = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let chars = char_pool.chars().collect::<Vec<char>>();
    let mut rng = ChaCha12Rng::from_os_rng();
    let mut rand_bytes = Vec::new();
    let cnt = chars.len();
    for _ in 0..length {
        let index = rng.random_range(0..cnt);
        rand_bytes.push(chars[index]);
    }
    rand_bytes.into_iter().collect()
}
