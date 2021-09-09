# RustでMD5を実装

[RFC1321](https://www.rfc-editor.org/rfc/inline-errata/rfc1321.html)に記述された仕様に従って[MD5](https://en.wikipedia.org/wiki/MD5)ハッシュアルゴリズムをRustで実装します。

- byteorder_demo: byteoderトレイトの使用例。
- diy_md5: メイン。MD5を実装。
- sin_brute_force: テイラーの定理を適用してsinを数値計算。
- sin_float: 浮動小数点数でsinを計算。
- sin_recursion: 加法定理と区間演算によって効率的にsinを数値計算。