use num::integer;
use std::ops::{Add, Mul, Sub};

//RFC1321記載の値
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

const RANGE_MAX: usize = 64;

macro_rules! frac_from_interval {
    [$min:expr, $max:expr] => [
        Frac::new(Interval::new($min, $max))
    ]
}

fn main() {
    let mut sin = vec![Option::<Frac>::None; RANGE_MAX + 1];
    let mut cos = vec![Option::<Frac>::None; RANGE_MAX + 1];
    sin[1] = Some(frac_from_interval![
        7761199951101802512,
        7761199951101802513
    ]);
    cos[1] = Some(frac_from_interval![
        4983409179392355912,
        4983409179392355913
    ]);

    let print_data = |i: i32, frac: &Frac| -> () {
        let t = frac.times_4294967296_to_u32();
        assert_eq!(t, T[i as usize]);
        println!("{0} {1:#x} {2}", i, t, frac.num.max - frac.num.min)
    };

    print_data(1, &sin[1].unwrap());

    for i in 2..=RANGE_MAX {
        let s1 = sin[i / 2].unwrap();
        let c1 = cos[i / 2].unwrap();
        let s2 = sin[(i + 1) / 2].unwrap();
        let c2 = cos[(i + 1) / 2].unwrap();
        sin[i] = Some(s1 * c2 + c1 * s2);
        cos[i] = Some(c1 * c2 - s1 * s2);
        print_data(i as i32, &sin[i].unwrap());
    }
}

//区間演算に用いる「区間」。
#[derive(Copy, Clone, Debug)]
struct Interval {
    min: i128,
    max: i128,
}

impl Interval {
    fn new(min: i128, max: i128) -> Interval {
        if min < 0 && 0 < max {
            panic!("Cannot contain 0.");
        } else if min > max {
            panic!("min = {} < max = {} is not satisfied.", min, max);
        }
        Interval { min, max }
    }

    fn signum(&self) -> i32 {
        if self.min > 0 && self.max > 0 {
            1
        } else if self.min < 0 && self.max < 0 {
            -1
        } else {
            panic!("Contains 0 {:?}", self);
        }
    }
}

//和差積の未実装。
impl Add for Interval {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Interval::new(self.min + rhs.min, self.max + rhs.max)
    }
}

impl Sub for Interval {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Interval::new(self.min - rhs.max, self.max - rhs.min)
    }
}

impl Mul for Interval {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let a = self.min;
        let b = self.max;
        let c = rhs.min;
        let d = rhs.max;
        let (min, max) = match (self.signum(), rhs.signum()) {
            (1, 1) => (a * c, b * d),
            (1, -1) => (b * c, a * d),
            (-1, 1) => (a * d, b * c),
            (-1, -1) => (b * d, a * c),
            _ => unreachable!(),
        };
        Interval::new(min, max)
    }
}

//2^63を分母に持ち、[-2^63,2^63]に含まれる区間を分母に持つ有理数。
#[derive(Debug, Copy, Clone)]
struct Frac {
    num: Interval, //denom=2^63
}

impl Frac {
    fn new(num: Interval) -> Self {
        if num.min < -(1 << 63) || 1 << 63 < num.max {
            panic!("Out of range")
        }
        Self { num }
    }

    //2^32倍の整数部分を返す
    fn times_4294967296_to_u32(&self) -> u32 {
        let min = integer::div_floor(self.num.min.abs(), 1 << 31);
        let max = integer::div_floor(self.num.max.abs(), 1 << 31);
        if min == max {
            min as u32
        } else {
            println!("{:?}", self.num);
            panic!("min={}, max={}", min, max);
        }
    }
}

//和差積のみ実装。
impl Add for Frac {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Frac::new(self.num + rhs.num)
    }
}

impl Sub for Frac {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Frac::new(self.num - rhs.num)
    }
}

impl Mul for Frac {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let num_prod = self.num * rhs.num;
        let floor = integer::div_floor(num_prod.min, 1 << 63);
        let ceil = integer::div_ceil(num_prod.max, 1 << 63);
        frac_from_interval![floor, ceil]
    }
}
