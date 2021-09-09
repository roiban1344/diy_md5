fn main(){
    //ひらがなと漢字を含む文字列
    let hash = diy_md5::md5("解けばわかる");
    println!("{:032x}", hash);
    assert_eq!(0x14980c8b8a96fd9e279796a61cf82c9c, hash);

    //表記揺れで全く異なる値になる
    let hash = diy_md5::md5("解けば分かる");
    println!("{:032x}", hash); //606461eb515ea6d825117fbe76965899

    //emoji
    let hash = diy_md5::md5("🐶");
    println!("{:032x}", hash); //be0f7766d0c41a4386d47e18e8b91e15
}