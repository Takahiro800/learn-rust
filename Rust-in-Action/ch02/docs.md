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