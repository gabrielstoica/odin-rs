#![allow(dead_code)]
#![allow(unused_variables)]
use elliptic_curve_math::{add, modular_inverse, multiply_scalar, Point};
use num_bigint::BigInt;

mod test;

/// secp256r1 curve parameters
/// Order
const N: &str = "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551";
/// Field
const P: &str = "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff";
/// Generator point
const G_X: &str = "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296";
const G_Y: &str = "4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5";
/// a coefficient
const A: &str = "ffffffff00000001000000000000000000000000fffffffffffffffffffffffc";
/// b coefficient
const B: &str = "5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b";

/// Q(x,y) - public key
/// message hash
/// [r, s] - signature
///
/// P1 = (s^-1)*message)*G
pub fn verify_signature_secp256r1(q: &Point, message: &BigInt, r: &BigInt, s: &BigInt) -> bool {
    let modulus: BigInt = BigInt::parse_bytes(N.as_bytes(), 16).unwrap();

    // Convert curve `a` field to BigInt
    let a: BigInt = BigInt::parse_bytes(A.as_bytes(), 16).unwrap();

    // Construct the generator point G
    let g = Point {
        x: BigInt::parse_bytes(G_X.as_bytes(), 16).unwrap(),
        y: BigInt::parse_bytes(G_Y.as_bytes(), 16).unwrap(),
    };

    let s_inverse: BigInt = modular_inverse(&s, &modulus);

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
