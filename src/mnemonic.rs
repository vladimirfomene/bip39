use language::{self, english };

mod util;

pub struct Mnemonic {
    pub lang: language::Language,
    pub words: Vec<&str>
}

impl Mnemonic {

    fn from_entropy_checksum(entropy: Vec<u8>, checksum: Vec<u8>) -> Mnemonic {
        let checksum_size = ((ent.entropy.len() * util::BYTE_SIZE) / 32);
        let entropy_size = entropy.len() * util::BYTE_SIZE;
        let mnemonic_word_count = (entropy_size + checksum_size) / 11;
        let mut bits = [false; (entropy_size + checksum_size)];

        //add entropy bits to bits array
        for (index, bit) in bits[..(entropy.len() * util::BYTE_SIZE)].iter_mut().enumerate(){
            *bit = util::get_index_bit(entropy[index / util::BYTE_SIZE], index % util::BYTE_SIZE);
        }

        //add checksum bits to bits array
        for (index, bit) in bits[(entropy.len() * util::BYTE_SIZE)..].iter_mut().enumerate() {
            *bit = util::get_index_bit(checksum[0], index);
        }

        //create 
        let mut mnemonic_list = Vec::with_capacity(mnemonic_word_count);
        for chunk in bits[..(checksum_size + entropy_size)].chunks(11) {
            //convert 11 bit chunk to word index
            let word_index = util::bits_to_usize(chunk, 11);

            //use word index to get word from wordlist
            let word = english::WORDS[word_index];

            //add word to mnemonic list
            mnemonic_list.push(word);
        }

        //return Mnemonic
        return Mnemonic {
            lang: Language::English,
            words: mnemonic_list
        }
        
    }


}