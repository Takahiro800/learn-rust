fn main() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    // aとbはビットパターンが同じだが、型が異なる
    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);
}
