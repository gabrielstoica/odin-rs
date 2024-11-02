#![allow(unused_imports)]
#![allow(dead_code)]
use super::*;

#[test]
fn test_verify_signature_secp256k1() {
    let r = BigInt::parse_bytes(
        "108607064596551879580190606910245687803607295064141551927605737287325610911759".as_bytes(),
        10,
    )
    .unwrap();

    let s = BigInt::parse_bytes(
        "42001087466938150539821028832855854854604982353441333885146378571977282687206".as_bytes(),
        10,
    )
    .unwrap();

    let q = Point {
        x: BigInt::parse_bytes(
            "33886286099813419182054595252042348742146950914608322024530631065951421850289"
                .as_bytes(),
            10,
        )
        .unwrap(),
        y: BigInt::parse_bytes(
            "9529752953487881233694078263953407116222499632359298014255097182349749987176"
                .as_bytes(),
            10,
        )
        .unwrap(),
    };

    let message = BigInt::parse_bytes(
        "103318048148376957923607078689899464500752411597387986125144636642406244063093".as_bytes(),
        10,
    )
    .unwrap();

    let modulus: BigInt = BigInt::parse_bytes(N.as_bytes(), 16).unwrap();
    assert_eq!(
        modulus,
        BigInt::parse_bytes(
            "115792089237316195423570985008687907852837564279074904382605163141518161494337"
                .as_bytes(),
            10
        )
        .unwrap()
    );

    // Convert curve `a` field to BigInt
    let a: BigInt = BigInt::parse_bytes(A.as_bytes(), 16).unwrap();

    // Construct the generator point G
    let g = Point {
        x: BigInt::parse_bytes(G_X.as_bytes(), 16).unwrap(),
        y: BigInt::parse_bytes(G_Y.as_bytes(), 16).unwrap(),
    };

    // ok
    let s_inverse: BigInt = modular_inverse(&s, &modulus);
    assert_eq!(
        s_inverse,
        BigInt::parse_bytes(
            "45447938250585175041485143783825456289541716944938209995676305366993446807883"
                .as_bytes(),
            10
        )
        .unwrap()
    );
    // ok
    let scalar_point_1 = (&s_inverse * message) % &modulus;
    assert_eq!(
        scalar_point_1,
        BigInt::parse_bytes(
            "115056011036436486431441801504780820141589077033987564458277636432076834079724"
                .as_bytes(),
            10
        )
        .unwrap()
    );

    // ok
    let scalar_point_2 = (&s_inverse * &r) % &modulus;
    assert_eq!(
        scalar_point_2,
        BigInt::parse_bytes(
            "24759105945829864362147675431322842137767494360198897137174760553021398405487"
                .as_bytes(),
            10
        )
        .unwrap()
    );

    let p1 = multiply_scalar(P, &a, &scalar_point_1, &g);
    assert_eq!(
        p1.x,
        BigInt::parse_bytes(
            "79261551546453979676536187227037311006680116096971555219245012641219881030394"
                .as_bytes(),
            10
        )
        .unwrap()
    );
    assert_eq!(
        p1.y,
        BigInt::parse_bytes(
            "26107055510772536427656396109878815731462032073255914648526184381963620705821"
                .as_bytes(),
            10
        )
        .unwrap()
    );

    let p2 = multiply_scalar(P, &a, &scalar_point_2, &q);
    assert_eq!(
        p2.x,
        BigInt::parse_bytes(
            "56330773661831478384892236439711913098730527993923776204734969254363484993989"
                .as_bytes(),
            10
        )
        .unwrap()
    );
    assert_eq!(
        p2.y,
        BigInt::parse_bytes(
            "78779774161647251292438626803690244436408340234975897171340339276147331374439"
                .as_bytes(),
            10
        )
        .unwrap()
    );

    let p3 = add(P, &a, &p1, &p2);
    assert_eq!(
        p3.x,
        BigInt::parse_bytes(
            "108607064596551879580190606910245687803607295064141551927605737287325610911759"
                .as_bytes(),
            10
        )
        .unwrap()
    );
    assert_eq!(
        p3.y,
        BigInt::parse_bytes(
            "109130787198476466480048840648958969424344577320182767582503142101362598828442"
                .as_bytes(),
            10
        )
        .unwrap()
    );

    assert_eq!(p3.x, r);
}
