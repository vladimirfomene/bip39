use entropy::Entropy;

pub mod entropy;

fn main() {
    

    //generate entropy (large random number)
    let ent = entropy::Entropy::new(256);
    println!("{:?}", ent);

    //generate checksum from entropy
    //Build mnemonic from entropy and wordlist
    //Generate seed from mnemonic using PBKDF2
}


fn generate_checksum(ent: Entropy) {
    //generate entropy from entropy
}
