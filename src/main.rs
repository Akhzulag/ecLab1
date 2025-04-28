use num_bigint::{BigInt as bui, ToBigInt};
use std::str::FromStr;

use num_bigint::RandBigInt;
use num_traits::{zero, FromPrimitive};
use num_traits::{One, Zero};
use rand::thread_rng;
mod ec;
use ec::{Point, EC};

fn main() {
    for _ in 0..1000 {
        let (p, ec) = EC::gen_point_p256();
        println!("{:?}", p);
        println!("{}", ec.on_curve(&p));
    }
}
