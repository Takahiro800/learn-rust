use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::net;
use std::net::Ipv6Addr;

// 列挙型を定義 　#[derive(Debug)]で注釈
#[derive(Debug)]
enum UpstreamError {
    IO(io::Error),
    Parsing(net::AddrParseError),
}

// std::fmt::Diplayを実装する
impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// std::error:Errorを実装する
impl error::Error for UpstreamError {} //デフォルトのメソッド実装に任せる。空白はコンパイラが埋めてくれる

fn main() -> Result<(), UpstreamError> {
    let _f = File::open("invisible.txt").map_err(UpstreamError::IO)?;

    let _localhost = "::1".parse::<Ipv6Addr>().map_err(UpstreamError::Parsing)?;

    Ok(())
}

/*
Error: IO(Os { code: 2, kind: NotFound, message: "No such file or directory" })
*/
