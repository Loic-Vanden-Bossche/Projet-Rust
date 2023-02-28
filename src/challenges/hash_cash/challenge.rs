use log::debug;
use md5::Digest;
use rand::Rng;
use super::types::{input::MD5HashCashInput, output::MD5HashCashOutput};

pub fn hash_cash(input: MD5HashCashInput) -> MD5HashCashOutput {
    debug!("complexity: {}\nmessage: {}", input.complexity, input.message);
    let mut rng = rand::thread_rng();
    let mut seed: u64;
    let answer: String;
    loop {
        seed = rng.gen::<u64>();
        let digest = md5::compute(format!("{seed:016x}{}", input.message.clone()));
        if count0(digest) >= input.complexity as u8 {
            answer = format!("{:X}", digest);
            break;
        }
    }
    MD5HashCashOutput { seed, hashcode: answer }
}

fn count0(digest: Digest) -> u32 {
    let str = format!("{:x}", digest);
    let value = match u128::from_str_radix(str.as_str(), 16) {
        Ok(val) => { val }
        Err(_) => { 0 }
    };
    return value.leading_zeros()
}