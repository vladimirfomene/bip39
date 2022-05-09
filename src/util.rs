const MIN_NUM_BITS: usize = 128;
const MAX_NUM_BITS: usize = 256;
const ENTROPY_MULTIPLE: usize = 32;
const BYTE_SIZE: usize = 8;

//remember index of binary numbers is counted from the left
//index here should be between 0 and 7.
pub fn get_index_bit(byte: u8, index: usize) -> bool {
    let mask = 1 << (BYTE_SIZE - 1 - index);
    return byte & mask > 0
}


pub fn bits_to_usize(chunk: &[bool], size: usize){
    let int = chunk
    .iter()
    .enumerate()
    .map(|(i, bit)| if *bit { 1 << (size - 1 - i) } else { 0 })
    .sum();

    return int as usize;
}