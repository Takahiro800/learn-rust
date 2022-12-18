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
* 静的メソッドのnew
    静的メソッドは、ある型で利用できるが、その型のインスタンスではない関数
どちらからというと new が好まれる。ライブラリの書き手は型のnew()を使ってデフォルトを設定するし、この方がコードが少しスッキリする
*/

// ここで、cargo add コマンドを利用できるように、cargo-editクレートをインストールした
/* cargo install cargo-edit
    これで、Cargo.tomlに依存対象を手作業で追加せずに、 cargo addコマンドで追加できる
*/
// 例）　cargo add num
