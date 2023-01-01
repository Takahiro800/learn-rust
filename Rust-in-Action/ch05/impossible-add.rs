// 明らかにオーバーフローを起こす状況をRustコンパイラは検出できる
// オーバーフローを許可する宣言が必要
#[allow(arithmetic_overflow)]
fn main() {
    let (a, b) = (200, 200);

    // この型宣言がないと、作ろうとしている不可能な状況を想定してくれない
    let c: u8 = a + b;
    println!("200 + 200 = {}", c);
}

/* 普通にコンパイルする
thread 'main' panicked at 'attempt to add with overflow', impossible-add.rs:8:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/

/* rustc -O impossible-add.rs
200 + 200 = 144
*/
