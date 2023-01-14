use std::error::Error;
use std::fs::File;
use std::net::Ipv6Addr;

fn main() -> Result<(), Box<dyn Error>> {
    let _f = File::open("invisible.txt")?;

    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}
/* 出力結果
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
*/

/* Note
Boxというラッパーで包む必要があるのは、そのサイズ（スタック上のバイト長）がコンパイル時に不明だから
トレイトオブジェクトは、File::open() または "::1".parse() のどちらかを起源とするが、それは実行時までわからない。
よって、ラッパーする必要がある
*/
