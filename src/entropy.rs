use rand::{self, RngCore};

const MIN_NUM_BITS: usize = 128;
const MAX_NUM_BITS: usize = 256;
const ENTROPY_MULTIPLE: usize = 32;
const BYTE_SIZE: usize = 8;

#[derive(Debug)]
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
        assert!((size >= MIN_NUM_BITS) && (size <= MAX_NUM_BITS));

        //check if size is a multiple of 32
        assert!((size % ENTROPY_MULTIPLE) == 0);
        let mut entropy: Vec<u8> = vec![0u8; size / BYTE_SIZE];
        //write better error handling code for this
        rand::rngs::OsRng.try_fill_bytes(&mut entropy).unwrap();
        

        return Entropy {
            entropy
        }
    }
}