use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::io::{Cursor, Read};

fn main() {
    assert_eq!(md5(""), 0xd41d8cd98f00b204e9800998ecf8427e);
    assert_eq!(md5("a"), 0x0cc175b9c0f1b6a831c399e269772661);
    assert_eq!(md5("abc"), 0x900150983cd24fb0d6963f7d28e17f72);
    assert_eq!(md5("message digest"), 0xf96b697d7cb7938d525a2f31aaf161d0);
    assert_eq!(
        md5("abcdefghijklmnopqrstuvwxyz"),
        0xc3fcd3d76192e4007dfb496cca67e13b
    );
    assert_eq!(
        md5("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"),
        0xd174ab98d277d9f5a5611c2c9f419d9f
    );
    assert_eq!(
        md5("12345678901234567890123456789012345678901234567890123456789012345678901234567890"),
        0x57edf4a22be3c955ac49da2e2107b67a
    );
}

const PADDING: [u8; 64] = [
    0b10000000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
];

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

fn f(x: u32, y: u32, z: u32) -> u32 {
    x & y | !x & z
}

fn g(x: u32, y: u32, z: u32) -> u32 {
    x & z | y & !z
}

fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn i(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}

fn md5(message: &str) -> u128 {
    let mut message = message.as_bytes().to_vec();
    let message_len_in_bytes = message.len();
    let mod_64 = message_len_in_bytes & 0b111111;
    let padding_len_in_bytes = if mod_64 < 56 {
        56 - mod_64
    } else {
        120 - mod_64
    };
    message.extend(&PADDING[..padding_len_in_bytes]);

    let mut buf: [u8; 8] = [0; 8];
    LittleEndian::write_u64(&mut buf, 8 * message_len_in_bytes as u64);
    message.extend(&buf);

    let mut a = 0x67452301u32;
    let mut b = 0xefcdab89u32;
    let mut c = 0x98badcfeu32;
    let mut d = 0x10325476u32;

    let mut cursor = Cursor::new(&message);
    let n = message.len() >> 6;

    for _ in 0..n {
        let mut block: [u8; 64] = [0; 64];
        cursor.read(&mut block).unwrap();
        let mut x: [u32; 16] = [0; 16];
        LittleEndian::read_u32_into(&block, &mut x);
        macro_rules! define_operation {
            ($macro_name: ident, $func: ident) => {
                macro_rules! $macro_name {
                    ($a:expr, $b:expr, $c:expr, $d: expr, $k:expr, $s:expr, $i:expr) => {
                        $a = $b.wrapping_add(
                            $a.wrapping_add($func($b, $c, $d))
                                .wrapping_add(x[$k])
                                .wrapping_add(T[$i])
                                .rotate_left($s),
                        )
                    };
                }
            };
        }
        let (aa, bb, cc, dd) = (a, b, c, d);
        define_operation!(ff, f);
        ff![a, b, c, d, 0, 7, 1];
        ff![d, a, b, c, 1, 12, 2];
        ff![c, d, a, b, 2, 17, 3];
        ff![b, c, d, a, 3, 22, 4];
        ff![a, b, c, d, 4, 7, 5];
        ff![d, a, b, c, 5, 12, 6];
        ff![c, d, a, b, 6, 17, 7];
        ff![b, c, d, a, 7, 22, 8];
        ff![a, b, c, d, 8, 7, 9];
        ff![d, a, b, c, 9, 12, 10];
        ff![c, d, a, b, 10, 17, 11];
        ff![b, c, d, a, 11, 22, 12];
        ff![a, b, c, d, 12, 7, 13];
        ff![d, a, b, c, 13, 12, 14];
        ff![c, d, a, b, 14, 17, 15];
        ff![b, c, d, a, 15, 22, 16];
        define_operation!(gg, g);
        gg![a, b, c, d, 1, 5, 17];
        gg![d, a, b, c, 6, 9, 18];
        gg![c, d, a, b, 11, 14, 19];
        gg![b, c, d, a, 0, 20, 20];
        gg![a, b, c, d, 5, 5, 21];
        gg![d, a, b, c, 10, 9, 22];
        gg![c, d, a, b, 15, 14, 23];
        gg![b, c, d, a, 4, 20, 24];
        gg![a, b, c, d, 9, 5, 25];
        gg![d, a, b, c, 14, 9, 26];
        gg![c, d, a, b, 3, 14, 27];
        gg![b, c, d, a, 8, 20, 28];
        gg![a, b, c, d, 13, 5, 29];
        gg![d, a, b, c, 2, 9, 30];
        gg![c, d, a, b, 7, 14, 31];
        gg![b, c, d, a, 12, 20, 32];
        define_operation!(hh, h);
        hh![a, b, c, d, 5, 4, 33];
        hh![d, a, b, c, 8, 11, 34];
        hh![c, d, a, b, 11, 16, 35];
        hh![b, c, d, a, 14, 23, 36];
        hh![a, b, c, d, 1, 4, 37];
        hh![d, a, b, c, 4, 11, 38];
        hh![c, d, a, b, 7, 16, 39];
        hh![b, c, d, a, 10, 23, 40];
        hh![a, b, c, d, 13, 4, 41];
        hh![d, a, b, c, 0, 11, 42];
        hh![c, d, a, b, 3, 16, 43];
        hh![b, c, d, a, 6, 23, 44];
        hh![a, b, c, d, 9, 4, 45];
        hh![d, a, b, c, 12, 11, 46];
        hh![c, d, a, b, 15, 16, 47];
        hh![b, c, d, a, 2, 23, 48];
        define_operation!(ii, i);
        ii![a, b, c, d, 0, 6, 49];
        ii![d, a, b, c, 7, 10, 50];
        ii![c, d, a, b, 14, 15, 51];
        ii![b, c, d, a, 5, 21, 52];
        ii![a, b, c, d, 12, 6, 53];
        ii![d, a, b, c, 3, 10, 54];
        ii![c, d, a, b, 10, 15, 55];
        ii![b, c, d, a, 1, 21, 56];
        ii![a, b, c, d, 8, 6, 57];
        ii![d, a, b, c, 15, 10, 58];
        ii![c, d, a, b, 6, 15, 59];
        ii![b, c, d, a, 13, 21, 60];
        ii![a, b, c, d, 4, 6, 61];
        ii![d, a, b, c, 11, 10, 62];
        ii![c, d, a, b, 2, 15, 63];
        ii![b, c, d, a, 9, 21, 64];
        a = a.wrapping_add(aa);
        b = b.wrapping_add(bb);
        c = c.wrapping_add(cc);
        d = d.wrapping_add(dd);
    }

    let mut buf = [0; 16];
    LittleEndian::write_u32_into(&[a, b, c, d], &mut buf);
    BigEndian::read_u128(&buf)
}
