# 2.2
- 文字列にはダブルクオートを使う。シングルクオートを使うのは `char`という別の型

## 2.6 参照を使う
- リスト2-10

```rust
fn main() {
    let a = 42;
    let r = &a;
    let b = a + *r;

    println!("a + a = {}", b);
}
```

参照は参照演算子(&)によって作られ、参照を介したアクセスはでリファレンス演算子(*)によって行われる。
この２つの演算子は、オペランドを１つしか取らない単項演算子。

**参照**は、他のある値を、アドレスによって参照するあたい。ある意味では安価なコピーだが、本当に複製するのではなく、アドレスをメモリに保存する。データを取り出す必要が生じたら、デリファレンス(dereference)で変数を作ることができる

## 2.8 より高度な関数定義
```rust:2-13｜明示的なライフタイム注釈を持つ関数シグネイチャ
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
	*i + *j
}
```
- `<'a, 'b>`は、add_with_lifetimes()をスコープとする２つのライフタイム変数、`'a`,''b'を宣言している。これらは「ライフタイムa」などと呼ばれる
- `i: &'a i32`は、ライフタイム変数`'a`を、iのライフタイムに束縛する。つまり、iは`ライフタイムaを持つ、i32型への参照`。

### ジェネリック関数
```rust:2-15｜ジェネリック関数のシグネチャ
// 型変数Tが、山括弧のペアとともに導入されている(<T>)
// 同じ型の引数を２つとり、その型の値を１つ返す
fn add<T>(i: T, j: T) -> T {
	i + j
}
```
- 任意の型に対して、加算が定義されているわけではないのでこのコードのコンパイルは失敗する
- 型Tには、加算の実装が必要だという条件を指定する必要がある。関数定義の中に、型変数とともに `トレイト境界(trait bound)`を入れなければならない。

```rust:2-16｜トレイト境界を持つジェネリック関数の型シグネチャ
fn add<T: std:ops:Add<Output = T>>(i: T, j: T) -> T {
	i + j
}
```
- `d<T: std:ops:Add<Output = T>>`は、「Tはstd:ops:Addを実装していなければダメ」という意味。
- `トレイト`という言語機能は、OOPで例えると抽象基底クラスのようなもの。Haskellの型クラスに近い。
- Rustの演算子は全て、トレイト付きで定義される。

```rust:2-17｜型変数とトレイト境界を持つジェネリック関数
use std:ops::{Add};
use std:time::{Duration};

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
	i + j
}

fn main() {
	let floats = add(1.2, 3.4);
	let ints = add(10, 20);
	let durations = add(
		Duration::new(5,0);
		Duration::new(10,0);
	);

	println!("{}", floats);
	println!("{}", ints);
	println!("{}", durations;);
}
```
- 関数addへの引数は、`std:ops::Add`を実装する型なら何でも受け入れる