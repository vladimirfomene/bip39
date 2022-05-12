use entropy::Entropy;
use language::english;
use mnemonic::Mnemonic;
use seed::compute_seed;
use bitcoin_hashes::{ sha256, Hash };

mod entropy;
mod language;
mod mnemonic;
mod util;
mod seed;

fn main() {
    

    //generate entropy (large random number)
    let ent = entropy::Entropy::new(256);
    println!("{:?}", ent);

    let checksum = generate_checksum(ent.clone());

    println!("{}, {:?}", checksum.len(), checksum);

    //println!("{:b}", checksum[0]);

    println!("{}", english::WORDS.len());

    let mnemonic = Mnemonic::from_entropy_checksum(ent.entropy, checksum);

    println!("{:?}", mnemonic);

    //generate checksum from entropy
    //Build mnemonic from entropy and wordlist
    let seed = compute_seed(mnemonic.words, "");
    println!("{:?}", seed);

    //Generate seed from mnemonic using PBKDF2
}


fn generate_checksum(ent: Entropy) -> Vec<u8> {
    //generate checksum from entropy
    let entropy_hash = sha256::Hash::hash(&ent.entropy);
    let entropy_hash = entropy_hash.as_ref();
    //multiply by 8 to convert from bytes to bits, divide by 32 as recommended in the BIP
    //This will give you the size of checksum in bits and now you divide by 8 again to 
    //convert to bytes
    //let checksum_size = ((ent.entropy.len() * 8) / 32) / 8;
    println!("{:?}", entropy_hash);
    return entropy_hash[0..1].to_owned();
}
