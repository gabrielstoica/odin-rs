#![allow(dead_code)]
#![allow(unused_variables)]
use elliptic_curve_math::{add, modular_inverse, multiply_scalar, Point};
use num_bigint::BigInt;

mod test;

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

/// Q(x,y) - public key
/// message hash
/// [r, s] - signature
///
/// P1 = (s^-1)*message)*G
pub fn verify_signature_secp256k1(q: &Point, message: &BigInt, r: &BigInt, s: &BigInt) -> bool {
    let modulus: BigInt = BigInt::parse_bytes(N.as_bytes(), 16).unwrap();

    // Convert curve `a` field to BigInt
    let a: BigInt = BigInt::parse_bytes(A.as_bytes(), 16).unwrap();

    // Construct the generator point G
    let g = Point {
        x: BigInt::parse_bytes(G_X.as_bytes(), 16).unwrap(),
        y: BigInt::parse_bytes(G_Y.as_bytes(), 16).unwrap(),
    };
    println!("generator: {:?}", g);
    // ok
    let s_inverse: BigInt = modular_inverse(&s, &modulus);
    println!("s_inverse: {}", s_inverse);

    // ok
    let scalar_point_1 = (&s_inverse * message) % &modulus;
    println!("scalar 1: {}", scalar_point_1);

    // ok
    let scalar_point_2 = (&s_inverse * r) % &modulus;
    println!("scalar 2: {}", scalar_point_2);

    let p1 = multiply_scalar(P, &a, &scalar_point_1, &g);
    println!("p1: {:?}", p1);
    let p2 = multiply_scalar(P, &a, &scalar_point_2, &q);
    println!("p2: {:?}", p2);

    let p3 = add(P, &a, &p1, &p2);

    println!("{:?}", p3);

    return p3.x == *r;
}
