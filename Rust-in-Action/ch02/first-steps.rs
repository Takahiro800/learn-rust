fn main() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(c, d));

    println!("(a + b) + (c + d) = {}", e);
}
// 文字列にはダブルクオートを使う。シングルクオートを使うのは `char`という別の型

fn add(i: i32, j: i32) -> i32 {
    i + j
}
