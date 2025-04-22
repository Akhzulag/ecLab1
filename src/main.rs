use num_bigint::{BigInt as bui, ToBigInt};
use std::str::FromStr;

use num_traits::{zero, FromPrimitive};
use num_traits::{One, Zero};
use rand::thread_rng;

use num_bigint::RandBigInt;
mod ec;

use ec::{Point, EC};

fn legendre_symbol(a: &bui, p: &bui) -> bui {
    let pow = p - &bui::one();
    let pow: bui = pow / 2;
    let r = a.modpow(&pow, p);
    if r == p - 1 {
        return -1.to_bigint().unwrap();
    }

    r
}

fn solve(x: bui, p: bui) -> bui {
    let mut q = &p - 1;
    let mut s = 0;
    while &q & bui::one() != bui::zero() {
        q >>= 1;
        s += 1;
    }

    if s == 1 {
        // let pow = &p + 1;
        // let pow = &pow >> 2;
        return x.modpow(&((&p + 1) >> 2), &p);
    }

    let mut rng = rand::thread_rng();
    let z = loop {
        let k: bui = rng.gen_bigint_range(&bui::one(), &q);
        if legendre_symbol(&k, &p) == -1.to_bigint().unwrap() {
            break k;
        }
    };

    let mut c = z.modpow(&q, &p);

    let mut r = x.modpow(&((&q + 1) >> 1), &p);

    let mut t = x.modpow(&q, &p);

    let mut m = s.clone();

    loop {
        if t == bui::one() {
            break r;
        }

        let mut i = 0;

        let i = loop {
            if t.modpow(&(2 >> i).to_bigint().unwrap(), &p) == bui::one() {
                break i;
            }
            i += 1;
        };

        let b = c.modpow(&((2 >> (m - i - 1)).to_bigint().unwrap()), &p);

        r = (r * &b) % &p;
        t = (t * b.pow(2)) % &p;
        c = b.pow(2) % &p;
        m = i;
    }
}

fn main() {
    let hex_str = "115792089210356248762697446949407573530086143415290314195533631308867097853951"; // шістнадцятковий рядок
    let big_int = bui::from_str(hex_str).expect("Invalid hex string");

    println!("BigInt value: {}", big_int);

    let n = bui::from(123456789u32);
    let n_str = n.to_str_radix(2); // Converts to string in base 10
    println!("BigUint as string: {:b}", n);

    let ec = EC::new(
        bui::from_str(
            "115792089210356248762697446949407573530086143415290314195533631308867097853948",
        )
        .unwrap(),
        bui::from_str(
            "41058363725152142129326129780047268409114441015993725554835256314039467401291",
        )
        .unwrap(),
        bui::from_str(
            "115792089210356248762697446949407573530086143415290314195533631308867097853951",
        )
        .unwrap(),
    );

    let g = Point::new(
        bui::from_str(
            "48439561293906451759052585252797914202762949526041747995844080717082404635286",
        )
        .unwrap(),
        bui::from_str(
            "36134250956749795798585127919587881956611106672985015071877198253568414405109",
        )
        .unwrap(),
        Some(bui::from_str("1").unwrap()),
    );

    let r = ec.scalar_mul(
        &g,
        bui::from_str(
            "115792089210356248762697446949407573529996955224135760342422259061068512044369",
        )
        .unwrap(),
    );

    println!("{:?}", &r);
    println!("{:?}", ec.convert(&r).unwrap());

    println!("{:?}", ec.on_curve(&g));
}
