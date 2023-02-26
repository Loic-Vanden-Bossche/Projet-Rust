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
        println!("{}, {}", count0(digest), input.complexity);
        if count0(digest) >= input.complexity as u8 {
            answer = format!("{:x}", digest);
            break;
        }
    }
    MD5HashCashOutput { seed, hashcode: answer }
}

fn count0(digest: Digest) -> u8 {
    let mut tot: u8 = 0;
    for i in 0..digest.0.len() {
        let mut tmp = 128;
        while digest[i] < tmp {
            tot += 1;
            tmp /= 2;
        }
        if tmp != 1 {
            break;
        }
    }
    return tot;
}