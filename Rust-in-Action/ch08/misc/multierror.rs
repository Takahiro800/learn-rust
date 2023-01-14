/// 複数の Result型を返そうとする関数
use std::fs::File;
use std::net::Ipv6Addr;

fn main() -> Result<(), std::io::Error> {
    // File::open が返すのは、Result<(), std::io::Error>
    let _f = File::open("invisible.txt")?;

    // "".parse::<Ipv6Addr>が返すのは、Result<Ipv6Addr, std::net::AddrParseError>
    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}

/* コンパイルエラー
error[E0277]: `?` couldn't convert the error to `std::io::Error`
               ↑ ?演算子が、エラーを std::io::Errorに変換できなかった
 --> multierror.rs:9:47

4 | fn main() -> Result<(), std::io::Error> {
  |              -------------------------- expected `std::io::Error` because of this
                                            std::io::Error が期待された
...
9 |     let _localhost = "::1".parse::<Ipv6Addr>()?;
  |                                               ^ the trait `From<AddrParseError>` is not implemented for `std::io::Error`
                                                std::io::Error のための From<AddrParseError>トレイトが実装されていない
  |
  = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
    ?演算子は、暗黙のうちにFromトレイトを使って、エラー値の変換を行う
  = help: the following other types implement trait `From<T>`:
            <std::io::Error as From<ErrorKind>>
            <std::io::Error as From<IntoInnerError<W>>>
            <std::io::Error as From<alloc::ffi::c_str::NulError>>
  = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, AddrParseError>>` for `Result<(), std::io::Error>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
*/
