const T: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x2441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x4881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

type Aux = fn(u32,u32,u32)->u32;

fn f(x: u32, y: u32, z: u32)->u32 {
    (x & y) | ((!x) & z)
}

fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & (!z))
}

fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn i(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}

fn main() {
    let x = vec![0b10000000,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

    let mut a = 0x67452301u32;
    let mut b = 0xefcdab89u32;
    let mut c = 0x98badcfeu32;
    let mut d = 0x10325476u32;

    let aa = a;
    let bb = b;
    let cc = c;
    let dd = d;

    println!("{:?}", x);

    let op = |a: &mut u32, &b: &u32, &c: &u32, &d: &u32, k: usize, s: u32, i: usize| {
        *a = b.wrapping_add(a.wrapping_add(f(b, c, d).wrapping_add(x[k]).wrapping_add(T[i - 1])).rotate_left(s));
    };

    op(&mut a, &b, &c, &d, 0, 7, 1);
    op(&mut d, &a, &b, &c, 1, 12, 2);
    op(&mut c, &d, &a, &b, 2, 17, 3);
    op(&mut b, &c, &d, &a, 3, 22, 4);
    op(&mut a, &b, &c, &d, 4, 7, 5);
    op(&mut d, &a, &b, &c, 5, 12, 6);
    op(&mut c, &d, &a, &b, 6, 17, 7);
    op(&mut b, &c, &d, &a, 7, 22, 8);
    op(&mut a, &b, &c, &d, 8, 7, 9);
    op(&mut d, &a, &b, &c, 9, 12, 10);
    op(&mut c, &d, &a, &b, 10, 17, 11);
    op(&mut b, &c, &d, &a, 11, 22, 12);
    op(&mut a, &b, &c, &d, 12, 7, 13);
    op(&mut d, &a, &b, &c, 13, 12, 14);
    op(&mut c, &d, &a, &b, 14, 17, 15);
    op(&mut b, &c, &d, &a, 15, 22, 16);

    let op = |a: &mut u32, &b: &u32, &c: &u32, &d: &u32, k: usize, s: u32, i: usize| {
        *a = b.wrapping_add(a.wrapping_add(g(b, c, d).wrapping_add(x[k]).wrapping_add(T[i - 1])).rotate_left(s));
    };

    op(&mut a, &b, &c, &d, 1, 5, 17);
    op(&mut d, &a, &b, &c, 6, 9, 18);
    op(&mut c, &d, &a, &b, 11, 14, 19);
    op(&mut b, &c, &d, &a, 0, 20, 20);
    op(&mut a, &b, &c, &d, 5, 5, 21);
    op(&mut d, &a, &b, &c, 10, 9, 22);
    op(&mut c, &d, &a, &b, 15, 14, 23);
    op(&mut b, &c, &d, &a, 4, 20, 24);
    op(&mut a, &b, &c, &d, 9, 5, 25);
    op(&mut d, &a, &b, &c, 14, 9, 26);
    op(&mut c, &d, &a, &b, 3, 14, 27);
    op(&mut b, &c, &d, &a, 8, 20, 28);
    op(&mut a, &b, &c, &d, 13, 5, 29);
    op(&mut d, &a, &b, &c, 2, 9, 30);
    op(&mut c, &d, &a, &b, 7, 14, 31);
    op(&mut b, &c, &d, &a, 12, 20, 32);

    let op = |a: &mut u32, &b: &u32, &c: &u32, &d: &u32, k: usize, s: u32, i: usize| {
        *a = b.wrapping_add(a.wrapping_add(h(b, c, d).wrapping_add(x[k]).wrapping_add(T[i - 1])).rotate_left(s));
    };
    op(&mut a, &b, &c, &d, 5, 4, 33);
    op(&mut d, &a, &b, &c, 8, 11, 34);
    op(&mut c, &d, &a, &b, 11, 16, 35);
    op(&mut b, &c, &d, &a, 14, 23, 36);
    op(&mut a, &b, &c, &d, 1, 4, 37);
    op(&mut d, &a, &b, &c, 4, 11, 38);
    op(&mut c, &d, &a, &b, 7, 16, 39);
    op(&mut b, &c, &d, &a, 10, 23, 40);
    op(&mut a, &b, &c, &d, 13, 4, 41);
    op(&mut d, &a, &b, &c, 0, 11, 42);
    op(&mut c, &d, &a, &b, 3, 16, 43);
    op(&mut b, &c, &d, &a, 6, 23, 44);
    op(&mut a, &b, &c, &d, 9, 4, 45);
    op(&mut d, &a, &b, &c, 12, 11, 46);
    op(&mut c, &d, &a, &b, 15, 16, 47);
    op(&mut b, &c, &d, &a, 2, 23, 48);

    let op = |a: &mut u32, &b: &u32, &c: &u32, &d: &u32, k: usize, s: u32, j: usize| {
        *a = b.wrapping_add(a.wrapping_add(i(b, c, d).wrapping_add(x[k]).wrapping_add(T[j - 1])).rotate_left(s));
    };
    op(&mut a, &b, &c, &d, 0, 6, 49);
    op(&mut d, &a, &b, &c, 7, 10, 50);
    op(&mut c, &d, &a, &b, 14, 15, 51);
    op(&mut b, &c, &d, &a, 5, 21, 52);
    op(&mut a, &b, &c, &d, 12, 6, 53);
    op(&mut d, &a, &b, &c, 3, 10, 54);
    op(&mut c, &d, &a, &b, 10, 15, 55);
    op(&mut b, &c, &d, &a, 1, 21, 56);
    op(&mut a, &b, &c, &d, 8, 6, 57);
    op(&mut d, &a, &b, &c, 15, 10, 58);
    op(&mut c, &d, &a, &b, 6, 15, 59);
    op(&mut b, &c, &d, &a, 13, 21, 60);
    op(&mut a, &b, &c, &d, 4, 6, 61);
    op(&mut d, &a, &b, &c, 11, 10, 62);
    op(&mut c, &d, &a, &b, 2, 15, 63);
    op(&mut b, &c, &d, &a, 9, 21, 64);

    a = a.wrapping_add(aa);
    b = b.wrapping_add(bb);
    c = c.wrapping_add(cc);
    d = d.wrapping_add(dd);

    println!("{:x?}",[a,b,c,d]);
    println!("{:x}", 0xd41d8cd9_8f00b204_e9800998_ecf8427eu128);
}
