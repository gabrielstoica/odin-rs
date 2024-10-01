extern crate crypto;

use self::crypto::sha3::Sha3;
use crypto::digest::Digest;
use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let address = &args[1];

    // validate if the provided address is a valid Ethereum address
    let regex = Regex::new(r"^0x[a-fA-F0-9]{40}$").unwrap();
    if !regex.is_match(&address) {
        println!("You have provided an invalid Ethereum address!");
        return;
    }

    // Computing the checksumm of an address is based on the following steps:
    // 1. Convert the address to lowercase and remove the 0x prefix
    // 2. Compute the keccak-256 hash of the lowercase address
    // 3. Check if the nth char of the lowercase address is alphabetic
    // 4. If so, check if the nth char of the hashed address is greater than 7
    // 5. If so, convert the nth char of the lowercase address to uppercase

    // convert the address to lowercase and remove the 0x prefix
    let lowercase_address = address.trim_start_matches("0x").to_lowercase();

    // get the keccak256 hash of the lowercase address
    let mut hasher = Sha3::keccak256();
    hasher.input_str(&lowercase_address);
    let address_hash = hasher.result_str();

    // loop through the address chars and check for conditions above
    let mut checksum_address = Vec::new();
    for i in 0..lowercase_address.len() {
        let mut nth_char_in_address = lowercase_address.chars().nth(i).unwrap();

        // check if the nth char in the address is alphabetic
        if nth_char_in_address.is_alphabetic() {
            // get the ascii representation of the nth char in the address hash
            let nth_char_in_hex_address = address_hash.chars().nth(i).unwrap() as u16;

            // check if it's greater than 7 (55 in ascii)
            if nth_char_in_hex_address > 55 {
                nth_char_in_address = nth_char_in_address.to_ascii_uppercase();
            }
        }

        // push the formatted char in the checksum address array
        checksum_address.push(nth_char_in_address);
    }

    // convert the result to String
    let final_address: String = checksum_address.into_iter().collect();
    println!("0x{}", final_address);
}
