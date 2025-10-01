use hmac::{Hmac, Mac};
use rand::rngs::OsRng;
use rand::{Rng, RngCore};
use sha3::Sha3_256;

type HmacSha256 = Hmac<Sha3_256>;

pub struct FairRandomResult {
    pub final_value: u32,
    pub morty_value: u32,
    pub rick_value: u32,
    pub key: [u8; 32],
}

pub fn run_protocol(max_exclusive: u32, rick_value: u32) -> FairRandomResult {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    let morty_value = rand::thread_rng().gen_range(0..max_exclusive);

    let mut mac = HmacSha256::new_from_slice(&key).expect("HMAC can take key of any size");
    mac.update(&morty_value.to_be_bytes());
    let hmac_result = mac.finalize().into_bytes();

    println!("Morty: HMAC={}", hex::encode_upper(hmac_result));

    let final_value = (morty_value + rick_value) % max_exclusive;

    FairRandomResult {
        final_value,
        morty_value,
        rick_value,
        key,
    }
}
