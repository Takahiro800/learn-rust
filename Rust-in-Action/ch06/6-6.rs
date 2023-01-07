// ヒープに変数を作る

fn main() {
    let a: i32 = 40; // スタック
    let b: Box<i32> = Box::new(60); // 60はヒープに置かれる

    println!("{} + {} = {}", a, b, a + *b);
}
