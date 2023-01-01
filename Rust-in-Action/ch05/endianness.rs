use std::mem::transmute;

fn main() {
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    // std::mem::transmute()はコンパイラに
    // 「引数を左辺の型（i32）に変換せよ」と依頼する
    let a: i32 = unsafe { transmute(big_endian) };
    let b: i32 = unsafe { transmute(little_endian) };

    println!("{} vs {}", a, b);
}

/* 出力が実装するマシンに依存する
ほとんどのマシン： x86-64/AMD64
    -573785174 vs -1430532899
少数：
    -1430532899 vs -573785174
*/
