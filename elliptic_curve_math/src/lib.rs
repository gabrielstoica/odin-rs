use num_bigint::BigInt;

mod test;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: BigInt,
    pub y: BigInt,
}

pub fn modular_inverse(a: &BigInt, modulus: &BigInt) -> BigInt {
    // Use the extended Euclidean algorithm to find the modular inverse
    // Fermat's Little Theorem
    a.modpow(&(modulus - BigInt::from(2)), modulus)
}

/// lambda = (q.y - p.y) / (q.x - p.x)
/// x_r = lambda^2 - p.x - q.x
/// y_r = lambda * (p.x - x_r) - p.y
pub fn add(modulus: &BigInt, a: &BigInt, p1: &Point, p2: &Point) -> Point {
    if p1.x == p2.x && p1.y == p2.y {
        return double(modulus, a, p1);
    }

    //let modulus: BigInt = BigInt::parse_bytes(field.as_bytes(), 16).unwrap();

    let inv = modular_inverse(&(&p1.x - &p2.x), &modulus);

    let lambda = ((&p1.y - &p2.y) * inv) % modulus;
    let x3 = (lambda.pow(2) - &p1.x - &p2.x) % modulus;
    // Note: in Rust, -2 mod 7 = -2, therefore we have to add modulus to the result to get
    // the correct positive coordinate (-2 + 7 = 5); this does not affect the result if the
    // coordinate is already positive, example:
    // (a + p) mod p == a mod p
    // 4 mod 7 = 4 -> 4 + 7 mod 7 = 4
    let mut y3 = ((lambda * (&p1.x - &x3) - &p1.y) % modulus) % modulus;
    if y3 < BigInt::from(0) {
        y3 += modulus;
    }

    // Use modulo again to ensure no negative coordinates are returned
    Point { x: x3, y: y3 }
}

/// lambda = (3 * x1^2 + a) / 2 * y1
/// r_x = lambda^2 - 2*x1
/// r_y = lambda * (x1 - r_x) - y1
pub fn double(modulus: &BigInt, a: &BigInt, p: &Point) -> Point {
    //let modulus: BigInt = BigInt::parse_bytes(field.as_bytes(), 16).unwrap();

    let lambda: BigInt = (((3 * p.x.modpow(&BigInt::from(2), &modulus)) + a)
        * modular_inverse(&(2 * &p.y), &modulus))
        % modulus;

    let x3: BigInt = (lambda.pow(2) - 2 * &p.x) % modulus;
    // Note: in Rust, -2 mod 7 = -2, therefore we have to add modulus to the result to get
    // the correct positive coordinate (-2 + 7 = 5); this does not affect the result if the
    // coordinate is already positive, example:
    // (a + p) mod p == a mod p
    // 4 mod 7 = 4 -> 4 + 7 mod 7 = 4
    let mut y3: BigInt = (lambda * (&p.x - &x3) - &p.y) % modulus;
    if y3 < BigInt::from(0) {
        y3 += modulus;
    }

    // Use modulo again to ensure no negative coordinates are returned
    Point { x: x3, y: y3 }
}

pub fn multiply_scalar(field: &BigInt, a: &BigInt, k: &BigInt, p: &Point) -> Point {
    let mut result = p.clone();
    let bit_length = k.bits();

    // Reverse loop as the number is represented in little endian
    // and we want to parse it in big endian
    // Example:
    // 5555 -> 1010110110011 (binary) in big endian
    // but without the reverse loop it will be parsed
    // as 1100110110101 (binary) in little endian
    for i in (0..bit_length - 1).rev() {
        result = double(&field, &a, &result);

        if k.bit(i) {
            result = add(&field, &a, &result, &p);
        }
    }

    result
}
