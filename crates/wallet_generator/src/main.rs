#![allow(dead_code)]
#![allow(unused_variables)]
use alloy::primitives::U256;
use rand::Rng;

#[derive(Debug)]
struct Point {
    x: U256,
    y: U256,
}

fn mod_inv(a: U256, prime: U256) -> U256 {
    // Use the extended Euclidean algorithm to find the modular inverse
    a.pow_mod(U256::from(prime - U256::from(2)), prime) // Fermat's Little Theorem
}

// lambda = (q.y - p.y) / (q.x - p.x)
// x_r = lambda^2 - p.x - q.x
// y_r = lambda * (p.x - x_r) - p.y
fn add(p: &Point, q: &Point) -> Point {
    let prime: U256 = U256::from_str_radix(
        "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
        16,
    )
    .unwrap();

    let lambda = ((q.y - p.y) * mod_inv(q.x - p.x, prime)) % prime;
    let r_x = (lambda.pow(U256::from(2)) - p.x - q.x) % prime;
    let r_y = (lambda * (p.x - r_x) - p.y) % prime;

    return Point { x: r_x, y: r_y };
}

// lambda = 3 * (x1^2) / 2 * y1
// r_x = lambda^2 - 2*x1
// r_y = lambda * (x1 - r_x) - y1
fn double(p: Point) -> Point {
    let prime: U256 = U256::from_str_radix(
        "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
        16,
    )
    .unwrap();

    let lambda =
        ((U256::from(3) * p.x.pow(U256::from(2))) * mod_inv(U256::from(2) * p.y, prime)) % prime;
    let r_x = (lambda.pow(U256::from(2)) - (U256::from(2) * p.x)) % prime;
    let r_y = (lambda * (p.x - r_x) - p.y) % prime;

    return Point { x: r_x, y: r_y };
}

fn multiply_scalar(k: U256, g: Point) -> Point {
    let mut q = Point {
        x: U256::ZERO,
        y: U256::ZERO,
    };
    let mut g_clone = g;

    for i in 0..256 {
        if k.bit(i) {
            q = add(&q, &g_clone);
        }

        g_clone = double(g_clone);
    }

    return q;
}

fn main() {
    // generate a random number k
    // multiply k with a predetermined point on the curve
    // called the generator point G -> resulting in another point on the curve
    // which is the public key K
    // K = k * G

    let mut rng = rand::thread_rng();
    let random_bytes: [u8; 32] = rng.gen::<[u8; 32]>();

    //let k = U256::from_be_bytes(random_bytes);
    let k = U256::from_str_radix(
        "f8f8a2f43c8376ccb0871305060d7b27b0554d2cc72bccf41b2705608452f315",
        16,
    )
    .unwrap();

    let g_x = U256::from_str_radix(
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        16,
    )
    .unwrap();

    let g_y = U256::from_str_radix(
        "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        16,
    )
    .unwrap();

    let g = Point { x: g_x, y: g_y };

    let pub_key = multiply_scalar(k, g);

    // serialize public key

    /*  let pub_key_x = U256::to_string(&pub_key.x);
       let pub_key_y = U256::to_string(&pub_key.y);

       let serialized_pub_key = format!("04{pub_key_x}{pub_key_y}");
    */

    // serialize public key as hexadecimal
    let pub_key_x = format!("{:064x}", pub_key.x); // 64 characters for 32 bytes
    let pub_key_y = format!("{:064x}", pub_key.y); // 64 characters for 32 bytes

    let serialized_pub_key = format!("04{}{}", pub_key_x, pub_key_y);

    println!("{:?}", serialized_pub_key);
}
