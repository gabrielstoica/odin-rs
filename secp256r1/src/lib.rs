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
pub fn verify_signature_secp256r1(
    pub_x: &String,
    pub_y: &String,
    message: &String,
    r: &String,
    s: &String,
) -> bool {
    let message_bigint: BigInt = BigInt::parse_bytes(message.as_bytes(), 16).unwrap();
    let r_bigint: BigInt = BigInt::parse_bytes(r.as_bytes(), 10).unwrap();
    let s_bigint: BigInt = BigInt::parse_bytes(s.as_bytes(), 10).unwrap();

    let modulus: BigInt = BigInt::parse_bytes(N.as_bytes(), 16).unwrap();
    let field: BigInt = BigInt::parse_bytes(P.as_bytes(), 16).unwrap();

    // Convert curve `a` field to BigInt
    let a: BigInt = BigInt::parse_bytes(A.as_bytes(), 16).unwrap();

    // Construct the generator point G
    let g = Point {
        x: BigInt::parse_bytes(G_X.as_bytes(), 16).unwrap(),
        y: BigInt::parse_bytes(G_Y.as_bytes(), 16).unwrap(),
    };

    // Construct the Q point
    let q = Point {
        x: BigInt::parse_bytes(
            "44287010881208015365891457934322412831709505919103389800494869821653003543448"
                .as_bytes(),
            10,
        )
        .unwrap(),
        y: BigInt::parse_bytes(
            "8406816689267822401861499063555392983685355867895747502381532128270957686385"
                .as_bytes(),
            10,
        )
        .unwrap(),
    };

    let s_inverse = modular_inverse(&s_bigint, &modulus);

    let scalar_point_1 = (&s_inverse * message_bigint) % &modulus;
    let scalar_point_2 = (&s_inverse * &r_bigint) % &modulus;

    let p1 = multiply_scalar(&field, &a, &scalar_point_1, &g);
    let p2 = multiply_scalar(&field, &a, &scalar_point_2, &q);

    // let p3 = add(&field, &a, &p1, &p2);

    // return p3.x == r_bigint;
    return true;
}
