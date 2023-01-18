/// 8-10: 常に入出力エラーを起こすRustプログラム
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    let _f = File::open("invisible.txt")?;

    Ok(())
}
