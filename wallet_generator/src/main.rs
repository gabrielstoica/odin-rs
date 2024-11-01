#![allow(dead_code)]
#![allow(unused_variables)]
use eliptic_curve_math::{multiply_scalar, Point};
use num_bigint::{BigInt, RandBigInt};
use rand::thread_rng;
use sha3::{Digest, Keccak256};
use std::env;

/// secp256k1 curve parameters
/// Order
const N: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";
/// Field
const P: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
/// Generator point
const G_X: &str = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
const G_Y: &str = "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8";
/// a coefficient
const A: &str = "0000000000000000000000000000000000000000000000000000000000000000";
/// b coefficient
const B: &str = "0000000000000000000000000000000000000000000000000000000000000007";

/// Generate a random number k.
/// Multiply k with the generator point G, resulting in another point on the curve
/// which is the public key K.
/// K = k * G
fn main() {
    let args: Vec<String> = env::args().collect();

    // Try to get the first argument which is the number of addresses to generate
    let arg = match args.get(1) {
        Some(val) => val,
        None => "1",
    };

    // Parse the string argument into integer
    let no = match arg.parse::<u64>() {
        Ok(val) => val,
        Err(e) => {
            println!("Unable to parse number from argument: {}", e);
            return;
        }
    };
    let no_of_addresses_to_generate = no as usize;

    // Define the total number of points on the curve
    let max_range = BigInt::parse_bytes(N.as_bytes(), 16).unwrap();

    // Create the random number generator
    let mut rng = thread_rng();

    // Convert curve `a` field to BigInt
    let a: BigInt = BigInt::parse_bytes(A.as_bytes(), 16).unwrap();

    for i in 0..no_of_addresses_to_generate {
        println!(
            "-------------------------------Address #{}-------------------------------",
            i + 1
        );

        // Generate a random number k in range 1 -> number of points on the curve (N)
        let k = rng.gen_bigint_range(&BigInt::from(1), &max_range);
        println!("Private key: {:x}", k);

        // Construct the generator point G
        let g = Point {
            x: BigInt::parse_bytes(G_X.as_bytes(), 16).unwrap(),
            y: BigInt::parse_bytes(G_Y.as_bytes(), 16).unwrap(),
        };

        // Compute the public key by multiplying the random number k with the generator point G
        let pub_key_point = multiply_scalar(P, &a, &k, &g);

        // Serialize public key as hexadecimal
        let pub_key_x = format!("{:06x}", pub_key_point.x);
        let pub_key_y: String = format!("{:06x}", pub_key_point.y);

        // 04 + x-coordinate (32 bytes/64 hex) + y-coordinate (32 bytes/64 hex)
        println!("Public key: {:?}", format!("{}{}", pub_key_x, pub_key_y));

        let mut pub_key_concat = [pub_key_x, pub_key_y].concat();

        // If the length of the public key is odd, prepend a "0" to make it even
        if pub_key_concat.len() % 2 != 0 {
            pub_key_concat = format!("0{}", pub_key_concat);
        }

        // Convert the concatenated hex string to raw bytes
        let pub_key_concat_as_bytes = hex::decode(pub_key_concat).expect("Decoding failed");

        // Hash the public key (X and Y coordinates concatenated) using keccak256
        let mut hasher = Keccak256::new();
        hasher.update(pub_key_concat_as_bytes);
        let result = hasher.finalize();

        // Get the last 20 bytes and display it in hex
        let address_bytes = &result[12..];
        println!("Ethereum address: 0x{}", hex::encode(address_bytes));

        println!("------------------------------------------------------------------------\n");
    }
}
