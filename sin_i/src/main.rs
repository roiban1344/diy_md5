use num::bigint::{BigInt, Sign};
use num::rational::BigRational;
use num::Signed;
use num::{One, Zero};
use std::result::Result;

const T: [u32; 65] = [
    0, 0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
    0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e,
    0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x2441453, 0xd8a1e681,
    0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9,
    0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60,
    0xbebfbc70, 0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x4881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8,
    0xc4ac5665, 0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d,
    0x85845dd1, 0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
    0xeb86d391,
];

fn main() {
    let c = BigRational::from_integer(BigInt::new(Sign::Plus, vec![0, 1])); //=4294967296
    for x in 1..=64 {
        for i in 0.. {
            let (l, r) = approx_range(x, i);
            if l.signum() != r.signum() {
                continue;
            }
            let a = num::abs(&c * &l).floor().to_integer();
            let b = num::abs(&c * &r).floor().to_integer();
            if a == b {
                let t = to_u32(&a).unwrap();
                assert_eq!(T[x as usize], t);
                println!("{} {:x} {}", x, t, i);
                break;
            }
        }
    }
}

fn to_u32(n: &BigInt) -> Result<u32, ()> {
    let (sign, v) = n.to_u32_digits();
    if sign != Sign::Plus || v.len() != 1 {
        Result::Err(())
    } else {
        Result::Ok(v[0])
    }
}

fn approx_range(x: u32, n: u32) -> (BigRational, BigRational) {
    let mut sum = BigRational::zero();
    for i in 0..=n {
        sum += match i % 4 {
            1 => BigRational::new(BigInt::new(Sign::Plus, vec![x]).pow(i), factorial(i)),
            3 => BigRational::new(BigInt::new(Sign::Minus, vec![x]).pow(i), factorial(i)),
            _ => BigRational::zero(),
        };
    }
    let err = num::abs(BigRational::new(
        BigInt::new(Sign::Plus, vec![x]).pow(n + 1),
        factorial(n + 1),
    ));
    (&sum - &err, &sum + &err)
}

fn factorial(n: u32) -> BigInt {
    if n == 0 || n == 1 {
        BigInt::one()
    } else if n > 1 {
        BigInt::new(Sign::Plus, vec![n]) * factorial(n - 1)
    } else {
        panic!();
    }
}
