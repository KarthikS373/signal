use cosmwasm_std::{Env, MessageInfo};

use secret_toolkit::crypto::{sha_256, Prng};

pub fn generate_anonymous_id(env: &Env, info: &MessageInfo, entropy_bytes: &[u8]) -> String {
    let entropy_len = 16 + info.sender.as_bytes().len() + entropy_bytes.len();

    let mut rng_entropy = Vec::with_capacity(entropy_len);

    rng_entropy.extend_from_slice(&env.block.time.nanos().to_be_bytes());
    rng_entropy.extend_from_slice(info.sender.as_bytes());
    rng_entropy.extend_from_slice(entropy_bytes);

    let mut rng = Prng::new(entropy_bytes, &rng_entropy);
    let rand_slice = rng.rand_bytes();
    let key = sha_256(&rand_slice);

    "".to_string()
}
