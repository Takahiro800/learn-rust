fn parity_bit(bytes: &[u8]) -> u8 {
    let mut n_ones: u32 = 0;

    for byte in bytes {
        let ones = byte.count_ones();
        n_ones += ones;
        println!("{} (0b{:08b})には、1のビットが {} 個ある", byte, byte, ones);
    }
    (n_ones % 2 == 0) as u8
}
fn main() {
    let abc = b"abc";
    println!("入力: {:?}", abc);
    println!("出力: {:08x}", parity_bit(abc));
    println!();

    let abcd = b"abcd";
    println!("入力: {:?}", abcd);
    println!("結果: {:08x}", parity_bit(abcd));
}

/* NOTE
Rustの整数型は、どれも count_ones()と count_zeros() のメソッドを持っている
{:08x} ８桁で表示する
 */
