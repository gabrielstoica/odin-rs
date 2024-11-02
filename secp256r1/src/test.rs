#![allow(unused_imports)]
#![allow(dead_code)]
use super::*;

#[test]
fn test_verify_signature_secp256r1() {
    let r = BigInt::parse_bytes(
        "68449023142751417849721717863618968034536915122161212084967858248511514843855".as_bytes(),
        10,
    )
    .unwrap();

    let s = BigInt::parse_bytes(
        "3488552624795641752530543084319869349275815004565195928595219571486160100262".as_bytes(),
        10,
    )
    .unwrap();

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

    let message = BigInt::parse_bytes(
        "815e09a2bd2fc002455e4f7e27ded6fe16b2d5fb64e794b1330baf43240426c2".as_bytes(),
        16,
    )
    .unwrap();

    let modulus: BigInt = BigInt::parse_bytes(N.as_bytes(), 16).unwrap();

    let field: BigInt = BigInt::parse_bytes(P.as_bytes(), 16).unwrap();

    // Convert curve `a` field to BigInt
    let a: BigInt = BigInt::parse_bytes(A.as_bytes(), 16).unwrap();

    // Construct the generator point G
    let g = Point {
        x: BigInt::parse_bytes(G_X.as_bytes(), 16).unwrap(),
        y: BigInt::parse_bytes(G_Y.as_bytes(), 16).unwrap(),
    };

    let s_inverse: BigInt = modular_inverse(&s, &modulus);

    let scalar_point_1 = (&s_inverse * message) % &modulus;

    let scalar_point_2 = (&s_inverse * &r) % &modulus;

    let p1 = multiply_scalar(&field, &a, &scalar_point_1, &g);

    let p2 = multiply_scalar(&field, &a, &scalar_point_2, &q);

    let p3 = add(&field, &a, &p1, &p2);

    assert_eq!(p3.x, r);
}
