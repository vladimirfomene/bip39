use hmac::Hmac;
use sha2::Sha512;
use unicode_normalization::UnicodeNormalization;
use crate::util::PBKDF2_BYTES;
use crate::util::PBKDF2_ITERATIONS;
use crate::util::SALT_PREFIX;
use pbkdf2;

pub (crate) fn compute_seed(mnemonic: Vec<String>, passphrase: &str) -> Vec<u8>{

    let mnemonic_sentence = mnemonic.join(" ");
    let normalized_mnemonic = mnemonic_sentence.nfkd().collect::<String>();
    let normalized_mnemonic = normalized_mnemonic.as_bytes();
    let salt = format!("{}{}", SALT_PREFIX, passphrase);
    let normalized_salt = salt.nfkd().collect::<String>();
    let normalized_salt = normalized_salt.as_bytes();
    let mut seed = [0u8; PBKDF2_BYTES];
    pbkdf2::pbkdf2::<Hmac<Sha512>>(normalized_mnemonic, normalized_salt, PBKDF2_ITERATIONS as u32, &mut seed);
    return seed.to_vec();
}