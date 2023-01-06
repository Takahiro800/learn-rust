//  生ポインタ *const Tを作る

fn main() {
    let a: i64 = 42;
    // 変数aへの参照を、、i64への定数型生ポインタへとキャスト
    let a_ptr = &a as *const i64;

    // 変数aの値と、そのメモリアドレスを出力
    println!("a: {} ({:p})", a, a_ptr);
}
