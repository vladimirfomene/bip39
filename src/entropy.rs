use rand::{self, RngCore};
use crate::util;

#[derive(Debug, Clone)]
pub struct Entropy{
    pub entropy: Vec<u8>
}

impl Entropy {

    //create new entropy (large random number)
    //entropy size here is in bits.
    pub fn new(size: usize) -> Entropy {
        //Probably change this to a better error handling mechanism if this has 
        //to be a library.

        //check if size is between 128 and 256
        assert!((size >= util::MIN_NUM_BITS) && (size <= util::MAX_NUM_BITS));

        //check if size is a multiple of 32
        assert!((size % util::ENTROPY_MULTIPLE) == 0);
        let mut entropy: Vec<u8> = vec![0u8; size / util::BYTE_SIZE];
        //write better error handling code for this
        rand::rngs::OsRng.try_fill_bytes(&mut entropy).unwrap();
        

        return Entropy {
            entropy
        }
    }
}