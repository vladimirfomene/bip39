use entropy::Entropy;
use language::english;
use mnemonic::Mnemonic;
use seed::compute_seed;
use sha2::{ Sha256, Digest };

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
    //multiply by 8 to convert from bytes to bits, divide by 32 as recommended in the BIP
    //This will give you the size of checksum in bits and now you divide by 8 again to 
    //convert to bytes
    let mut hasher = Sha256::new();
    hasher.update(&ent.entropy);
    let entropy_hash = hasher.finalize();
    
    println!("{:?}", entropy_hash);
    return entropy_hash[0..1].to_owned();
}
