#![allow(dead_code)]
#![allow(unused_variables)]
use num_bigint::{BigInt, RandBigInt};
use rand::thread_rng;
use sha3::{Digest, Keccak256};

mod test;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: BigInt,
    pub y: BigInt,
}

fn modular_inverse(a: BigInt, modulus: &BigInt) -> BigInt {
    // Use the extended Euclidean algorithm to find the modular inverse
    // Fermat's Little Theorem
    a.modpow(&BigInt::from(modulus - BigInt::from(2)), &modulus)
}

/// lambda = (q.y - p.y) / (q.x - p.x)
/// x_r = lambda^2 - p.x - q.x
/// y_r = lambda * (p.x - x_r) - p.y
fn add(p1: &Point, p2: &Point) -> Point {
    if p1.x == p2.x && p1.y == p2.y {
        return double(p1.clone());
    }

    let modulus: BigInt = BigInt::parse_bytes(P.as_bytes(), 16).unwrap();

    let inv = modular_inverse(&p1.x - &p2.x, &modulus);

    let lambda = ((&p1.y - &p2.y) * inv) % &modulus;
    let x3 = (lambda.pow(2) - &p1.x - &p2.x) % &modulus;
    let y3 = (lambda * (&p1.x - &x3) - &p1.y) % &modulus;

    return Point { x: x3, y: y3 };
}

/// lambda = 3 * (x1^2) / 2 * y1
/// r_x = lambda^2 - 2*x1
/// r_y = lambda * (x1 - r_x) - y1
fn double(p: Point) -> Point {
    let modulus: BigInt = BigInt::parse_bytes(P.as_bytes(), 16).unwrap();

    let lambda: BigInt = ((3 * p.x.modpow(&BigInt::from(2), &modulus))
        * modular_inverse(2 * &p.y, &modulus))
        % &modulus;

    let x3: BigInt = (lambda.pow(2) - 2 * &p.x) % &modulus;
    let y3: BigInt = (lambda * (&p.x - &x3) - p.y) % &modulus;

    return Point { x: x3, y: y3 };
}

fn multiply_scalar(k: BigInt, p: Point) -> Point {
    let mut result = p.clone();
    let bit_length = k.bits();

    // Reverse loop as the number is represented in little endian
    // and we want to parse it in big endian
    // Example:
    // 5555 -> 1010110110011 (binary) in big endian
    // but without the reverse loop it will be parsed
    // as 1100110110101 (binary) in little endian
    for i in (0..bit_length - 1).rev() {
        result = double(result);

        if k.bit(i) {
            result = add(&result, &p);
        }
    }

    return result;
}

/// secp256k1 curve parameters
/// Order
const N: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";
/// Field
const P: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
/// Generator point
const G_X: &str = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
const G_Y: &str = "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8";

/// Generate a random number k.
/// Multiply k with the generator point G, resulting in another point on the curve
/// which is the public key K.
/// K = k * G
fn main() {
    // Define the total number of points on the curve
    let max_range = BigInt::parse_bytes(N.as_bytes(), 16).unwrap();

    // Create the random number generator
    let mut rng = thread_rng();

    // Generate a random number k in range 1 -> number of points on the curve (N)
    let k = rng.gen_bigint_range(&BigInt::from(1), &max_range);

    // Construct the generator point G
    let g = Point {
        x: BigInt::parse_bytes(G_X.as_bytes(), 16).unwrap(),
        y: BigInt::parse_bytes(G_Y.as_bytes(), 16).unwrap(),
    };

    // Compute the public key by multiplying the random number k with the generator point G
    let pub_key_point = multiply_scalar(k, g);

    // Serialize public key as hexadecimal and add `0x04` prefix
    let pub_key_x = format!("{:064x}", pub_key_point.x);
    let pub_key_y: String = format!("{:064x}", pub_key_point.y);

    // 04 + x-coordinate (32 bytes/64 hex) + y-coordinate (32 bytes/64 hex)
    // println!("{:?}", format!("04{}{}", pub_key_x, pub_key_y));

    let pub_key_concat = [pub_key_x, pub_key_y].concat();

    // Convert the concatenated hex string to raw bytes
    let pub_key_concat_as_bytes = hex::decode(pub_key_concat).expect("Decoding failed");

    // Hash the public key (X and Y coordinates concatenated) using keccak256
    let mut hasher = Keccak256::new();
    hasher.update(pub_key_concat_as_bytes);
    let result = hasher.finalize();

    // Get the last 20 bytes and display it in hex
    let address_bytes = &result[12..];
    println!("{}", format!("0x{}", hex::encode(result)));
}
