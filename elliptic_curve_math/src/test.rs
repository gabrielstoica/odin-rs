#![allow(unused_imports)]
#![allow(dead_code)]
use super::*;

/// secp256k1 curve parameters
/// y^2 = x^3 + a*x + b
///
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

#[test]
fn test_add() {
    let p1 = Point {
        x: BigInt::parse_bytes(
            "44581723464407256412309220680195319617498075473560654710249472444070677434403"
                .as_bytes(),
            10,
        )
        .unwrap(),
        y: BigInt::parse_bytes(
            "19283459354363003827793739044717556150832640647625551052404873467366856666095"
                .as_bytes(),
            10,
        )
        .unwrap(),
    };

    let p2 = Point {
        x: BigInt::parse_bytes(
            "21832994301515074090218006224743185419686457244009131704326103556774453935895"
                .as_bytes(),
            10,
        )
        .unwrap(),
        y: BigInt::parse_bytes(
            "60399087494983030623697227849582240398983581638306679283005823969766000456299"
                .as_bytes(),
            10,
        )
        .unwrap(),
    };

    let expected = Point {
        x: BigInt::parse_bytes(
            "77395458902313273853554392134346119134215153403287066751169005628917840598133"
                .as_bytes(),
            10,
        )
        .unwrap(),
        y: BigInt::parse_bytes(
            "33047341142863558735842438196201278199260712499670054188459255076092033889119"
                .as_bytes(),
            10,
        )
        .unwrap(),
    };
    println!("{:?}", expected);

    // Convert curve `a` field to BigInt
    let a: BigInt = BigInt::parse_bytes(A.as_bytes(), 16).unwrap();

    let actual = add(P, &a, &p1, &p2);
    assert_eq!(actual.x, expected.x);
}

#[test]
fn test_double() {
    let p1 = Point {
        x: BigInt::parse_bytes(
            "55066263022277343669578718895168534326250603453777594175500187360389116729240"
                .as_bytes(),
            10,
        )
        .unwrap(),
        y: BigInt::parse_bytes(
            "32670510020758816978083085130507043184471273380659243275938904335757337482424"
                .as_bytes(),
            10,
        )
        .unwrap(),
    };

    let expected = Point {
        x: BigInt::parse_bytes(
            "89565891926547004231252920425935692360644145829622209833684329913297188986597"
                .as_bytes(),
            10,
        )
        .unwrap(),
        y: BigInt::parse_bytes(
            "12158399299693830322967808612713398636155367887041628176798871954788371653930"
                .as_bytes(),
            10,
        )
        .unwrap(),
    };
    println!("{:?}", expected);

    // Convert curve `a` field to BigInt
    let a: BigInt = BigInt::parse_bytes(A.as_bytes(), 16).unwrap();

    let actual = double(&P, &a, &p1);
    assert_eq!(actual.x, expected.x);
}

#[test]
fn test_inverse() {
    let modulus: BigInt = BigInt::from(73);
    let a: BigInt = BigInt::from(17);

    let inverse = modular_inverse(&a, &modulus);
    assert_eq!(inverse, BigInt::from(43));
}

#[test]
fn test_multiply_scalar() {
    let g = Point {
        x: BigInt::parse_bytes(
            "85013610161300501634207097904060921905061818080704392857133632721418945641922"
                .as_bytes(),
            10,
        )
        .unwrap(),
        y: BigInt::parse_bytes(
            "51484713442008819730592617749735375257932097210801086371214757216985020248059"
                .as_bytes(),
            10,
        )
        .unwrap(),
    };

    let k = BigInt::parse_bytes(
        "51484713442008819730592617749735375257932097210801086371214757216985020248059".as_bytes(),
        10,
    )
    .unwrap();

    let expected = Point {
        x: BigInt::parse_bytes(
            "72352447593799398445922740549399987159431177238119866962767582363186985451835"
                .as_bytes(),
            10,
        )
        .unwrap(),
        y: BigInt::parse_bytes(
            "70185681729751987328402969426694159415632315296271757809744557899295792409494"
                .as_bytes(),
            10,
        )
        .unwrap(),
    };

    // Convert curve `a` field to BigInt
    let a: BigInt = BigInt::parse_bytes(A.as_bytes(), 16).unwrap();

    let actual = multiply_scalar(P, &a, &k, &g);
    assert_eq!(actual.x, expected.x);
}

/// See https://github.com/ethereumbook/ethereumbook/blob/develop/04keys-addresses.asciidoc#generating-a-public-key
#[test]
fn test_multiply_big_scalar() {
    let p = Point {
        x: BigInt::parse_bytes(
            "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798".as_bytes(),
            16,
        )
        .unwrap(),
        y: BigInt::parse_bytes(
            "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8".as_bytes(),
            16,
        )
        .unwrap(),
    };

    let k = BigInt::parse_bytes(
        "f8f8a2f43c8376ccb0871305060d7b27b0554d2cc72bccf41b2705608452f315".as_bytes(),
        16,
    )
    .unwrap();

    let expected = Point {
        x: BigInt::parse_bytes(
            "6e145ccef1033dea239875dd00dfb4fee6e3348b84985c92f103444683bae07b".as_bytes(),
            16,
        )
        .unwrap(),
        y: BigInt::parse_bytes(
            "83b5c38e5e2b0c8529d7fa3f64d46daa1ece2d9ac14cab9477d042c84c32ccd0".as_bytes(),
            16,
        )
        .unwrap(),
    };

    // Convert curve `a` field to BigInt
    let a: BigInt = BigInt::parse_bytes(A.as_bytes(), 16).unwrap();

    let actual = multiply_scalar(P, &a, &k, &p);
    assert_eq!(actual.x, expected.x);
    assert_eq!(actual.y, expected.y);
}

#[test]
fn test_multiply_big_scalar2() {
    let p = Point {
        x: BigInt::parse_bytes(
            "11557031742056114838734743550150354972631190840499583211275771016628870935876"
                .as_bytes(),
            10,
        )
        .unwrap(),
        y: BigInt::parse_bytes(
            "20930594233534484674627279732655251584241696903065387055974955470696465282724"
                .as_bytes(),
            10,
        )
        .unwrap(),
    };

    let k = BigInt::parse_bytes(
        "11101893574081133653551597477754358047597243130607845565088874785568729948118".as_bytes(),
        10,
    )
    .unwrap();

    let expected = Point {
        x: BigInt::parse_bytes(
            "23466405427901064848765405310743290764804626857592401332662890160966721856561"
                .as_bytes(),
            10,
        )
        .unwrap(),
        y: BigInt::parse_bytes(
            "78018453005541465285293987897263501197917348599982509589975751894960449664440"
                .as_bytes(),
            10,
        )
        .unwrap(),
    };

    // Convert curve `a` field to BigInt
    let a: BigInt = BigInt::parse_bytes(A.as_bytes(), 16).unwrap();

    let actual = multiply_scalar(P, &a, &k, &p);
    assert_eq!(actual.x, expected.x);
    assert_eq!(actual.y, expected.y);
}
