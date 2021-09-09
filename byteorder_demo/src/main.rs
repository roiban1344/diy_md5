use byteorder::{BigEndian, ByteOrder, LittleEndian};

fn main() {
    let bytes: [u8; 12] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x11, 0x22, 0x33, 0x44,
    ];
    let mut words: [u32; 3] = [0; 3];
    LittleEndian::read_u32_into(&bytes, &mut words);
    println!("{:x?}", words);
    assert_eq!(words[0], 0x_67_45_23_01);
    assert_eq!(words[1], 0x_ef_cd_ab_89);
    assert_eq!(words[2], 0x_44_33_22_11);

    BigEndian::read_u32_into(&bytes, &mut words);
    println!("{:x?}", words);
    assert_eq!(words[0], 0x_01_23_45_67);
    assert_eq!(words[1], 0x_89_ab_cd_ef);
    assert_eq!(words[2], 0x_11_22_33_44);
}
