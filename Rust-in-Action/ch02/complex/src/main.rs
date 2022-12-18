use num::complex::Complex; // useキーワードでComplex型をローカルスコープにインポートする

fn main() {
    let a = Complex { re: 2.1, im: -1.2 }; //　どのRust型にもリテラル型が存在する
    let b = Complex::new(11.1, 22.2); // ほとんどの型で静的メソッド new()が実装されている
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}

/* 出力結果
13.2 + 21i
*/

/*
* Rustにはコンストラクタがないが、どの型にもリテラル形式が存在する。
    型の名前（Complex）の後に、波括弧で囲んだフィールド(re, im)を並べ、それらに値を代入すれば、その型を初期化できる
*/
