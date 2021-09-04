use byteorder::{BigEndian, LittleEndian, ReadBytesExt, BE, LE};
use std::io::Cursor;

const t: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x2441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x4881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

fn f(x: u32, y: u32, z: u32) -> u32 {
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
    // let mut rdr = Cursor::new(vec![0, 2, 0, 3, 0, 4, 0, 5]);
    // println!("{:?}", rdr.read_u16::<BigEndian>().unwrap());
    // println!("{:?}", rdr.read_u16::<BigEndian>().unwrap());

    // println!("{:?}", rdr.read_u16::<LittleEndian>().unwrap());
    // println!("{:?}", rdr.read_u16::<LittleEndian>().unwrap());

    // println!("{:b}", 0b10001u32.rotate_left(2));

    let head = 0x00;

    let v: Vec<u8> = vec![
        head, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    let mut reader = Cursor::new(v); //?7

    let mut x: Vec<u32> = vec![];
    for _ in 0..16 {
        x.push(reader.read_u32::<BE>().unwrap());
    }

    // x = vec![
    //     0b10000000000000000000000000000000u32,
    //     0b00000000000000000000000000000000u32,
    //     0b00000000000000000000000000000000u32,
    //     0b00000000000000000000000000000000u32,
    // ];
    x = vec![0;16];
    //x[0] = 0b10000000000000000000000000000000u32;
    //x[0] = 0b1u32<<31;
    x[0] = 0x80;

    let mut a = 0x67452301u32;
    let mut b = 0xefcdab89u32;
    let mut c = 0x98badcfeu32;
    let mut d = 0x10325476u32;

    let aa = a;
    let bb = b;
    let cc = c;
    let dd = d;

    //let mut buf = vec![1].extend(vec![0;63]);
    //BigEndian::read_u32(&mut buf);

    println!("{:?}", x);

    //xのキャプチャ
    //&mut aとは書けない
    let round_1 = |a: &mut u32, &b: &u32, &c: &u32, &d: &u32, k: usize, s: u32, i: usize| {
        *a = b + (*a + f(b, c, d) + x[k] + t[i - 1]).rotate_left(s);
    };

    //\[([abcd])([abcd])([abcd])([abcd])\s+(\d+)\s+(\d+)\s+(\d+)\]

    round_1(&mut a, &b, &c, &d, 0, 7, 1);
    round_1(&mut d, &a, &b, &c, 1, 12, 2);
    round_1(&mut c, &d, &a, &b, 2, 17, 3);
    round_1(&mut b, &c, &d, &a, 3, 22, 4);
    round_1(&mut a, &b, &c, &d, 4, 7, 5);
    round_1(&mut d, &a, &b, &c, 5, 12, 6);
    round_1(&mut c, &d, &a, &b, 6, 17, 7);
    round_1(&mut b, &c, &d, &a, 7, 22, 8);
    round_1(&mut a, &b, &c, &d, 8, 7, 9);
    round_1(&mut d, &a, &b, &c, 9, 12, 10);
    round_1(&mut c, &d, &a, &b, 10, 17, 11);
    round_1(&mut b, &c, &d, &a, 11, 22, 12);
    round_1(&mut a, &b, &c, &d, 12, 7, 13);
    round_1(&mut d, &a, &b, &c, 13, 12, 14);
    round_1(&mut c, &d, &a, &b, 14, 17, 15);
    round_1(&mut b, &c, &d, &a, 15, 22, 16);

    let round_2 = |a: &mut u32, &b: &u32, &c: &u32, &d: &u32, k: usize, s: u32, i: usize| {
        *a = b + (*a + g(b, c, d) + x[k] + t[i - 1]).rotate_left(s);
    };
    round_2(&mut a, &b, &c, &d, 1, 5, 17);
    round_2(&mut d, &a, &b, &c, 6, 9, 18);
    round_2(&mut c, &d, &a, &b, 11, 14, 19);
    round_2(&mut b, &c, &d, &a, 0, 20, 20);
    round_2(&mut a, &b, &c, &d, 5, 5, 21);
    round_2(&mut d, &a, &b, &c, 10, 9, 22);
    round_2(&mut c, &d, &a, &b, 15, 14, 23);
    round_2(&mut b, &c, &d, &a, 4, 20, 24);
    round_2(&mut a, &b, &c, &d, 9, 5, 25);
    round_2(&mut d, &a, &b, &c, 14, 9, 26);
    round_2(&mut c, &d, &a, &b, 3, 14, 27);
    round_2(&mut b, &c, &d, &a, 8, 20, 28);
    round_2(&mut a, &b, &c, &d, 13, 5, 29);
    round_2(&mut d, &a, &b, &c, 2, 9, 30);
    round_2(&mut c, &d, &a, &b, 7, 14, 31);
    round_2(&mut b, &c, &d, &a, 12, 20, 32);

    let round_3 = |a: &mut u32, &b: &u32, &c: &u32, &d: &u32, k: usize, s: u32, i: usize| {
        *a = b + (*a + h(b, c, d) + x[k] + t[i - 1]).rotate_left(s);
    };
    round_3(&mut a, &b, &c, &d, 5, 4, 33);
    round_3(&mut d, &a, &b, &c, 8, 11, 34);
    round_3(&mut c, &d, &a, &b, 11, 16, 35);
    round_3(&mut b, &c, &d, &a, 14, 23, 36);
    round_3(&mut a, &b, &c, &d, 1, 4, 37);
    round_3(&mut d, &a, &b, &c, 4, 11, 38);
    round_3(&mut c, &d, &a, &b, 7, 16, 39);
    round_3(&mut b, &c, &d, &a, 10, 23, 40);
    round_3(&mut a, &b, &c, &d, 13, 4, 41);
    round_3(&mut d, &a, &b, &c, 0, 11, 42);
    round_3(&mut c, &d, &a, &b, 3, 16, 43);
    round_3(&mut b, &c, &d, &a, 6, 23, 44);
    round_3(&mut a, &b, &c, &d, 9, 4, 45);
    round_3(&mut d, &a, &b, &c, 12, 11, 46);
    round_3(&mut c, &d, &a, &b, 15, 16, 47);
    round_3(&mut b, &c, &d, &a, 2, 23, 48);

    let round_4 = |a: &mut u32, &b: &u32, &c: &u32, &d: &u32, k: usize, s: u32, j: usize| {
        *a = b + (*a + i(b, c, d) + x[k] + t[j - 1]).rotate_left(s);
    };
    round_4(&mut a, &b, &c, &d, 0, 6, 49);
    round_4(&mut d, &a, &b, &c, 7, 10, 50);
    round_4(&mut c, &d, &a, &b, 14, 15, 51);
    round_4(&mut b, &c, &d, &a, 5, 21, 52);
    round_4(&mut a, &b, &c, &d, 12, 6, 53);
    round_4(&mut d, &a, &b, &c, 3, 10, 54);
    round_4(&mut c, &d, &a, &b, 10, 15, 55);
    round_4(&mut b, &c, &d, &a, 1, 21, 56);
    round_4(&mut a, &b, &c, &d, 8, 6, 57);
    round_4(&mut d, &a, &b, &c, 15, 10, 58);
    round_4(&mut c, &d, &a, &b, 6, 15, 59);
    round_4(&mut b, &c, &d, &a, 13, 21, 60);
    round_4(&mut a, &b, &c, &d, 4, 6, 61);
    round_4(&mut d, &a, &b, &c, 11, 10, 62);
    round_4(&mut c, &d, &a, &b, 2, 15, 63);
    round_4(&mut b, &c, &d, &a, 9, 21, 64);

    a += aa;
    b += bb;
    c += cc;
    d += dd;

    // let reader = Cursor::new(vec![a,b,c,d]);
    // let mut v:Vec<u8> = vec![];
    // LE::write_u32_into(&[a,b,c,d], &mut v);

    println!("{:x?}",[a,b,c,d]);
    //println!("{:x}",0x68b329da9893e34099c7d8ad5cb9c940u128);    
    println!("{:x}", 0xd41d8cd9_8f00b204_e9800998_ecf8427eu128);
    
    println!("{:b}{:b}{:b}{:b}", a,b,c,d);
    println!("{:b}", 0xd41d8cd9_8f00b204_e9800998_ecf8427eu128);
    // println!("{:b} {:b} {:b} {:b}", 
    // 0x68b329dau32,
    // 0x9893e340u32,
    // 0x99c7d8adu32,
    // 0x5cb9c940u32);

    //$ md5sum <<< ""
    //68b329da9893e34099c7d8ad5cb9c940 *-
}
