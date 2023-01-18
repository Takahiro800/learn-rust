use std::io::Write;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let host = "www.rustinaction.com:80"; //ポート番号を明示的に指定

    // TODO ?とは
    let mut conn = TcpStream::connect(host)?;

    // HTTP1.0を指定。このバージョンは、サーバーは応答を送ったら接続を閉じる。
    // 1.1にすると、このコードはうまく動かない。キープアライブリクエストをサポートしているが、このコードは受信後にコードを送らないため。
    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(b"\r\n")?; //改行

    // HTTPヘッダとして、Hostを追加。コンテクストとして補充するため。
    conn.write_all(b"Host: www.rustinaction.com")?;
    conn.write_all(b"\r\n\r\n")?; //改行２つでリクエストの終わりを示す

    std::io::copy(&mut conn, &mut std::io::stdout())?;

    Ok(())
}

/*
HTTP/1.0 301 Moved Permanently
Content-Type: text/html; charset=utf-8
Location: https://www.rustinaction.com/
Permissions-Policy: interest-cohort=()
Vary: Origin
Date: Fri, 13 Jan 2023 20:52:42 GMT
Content-Length: 64

<a href="https://www.rustinaction.com/">Moved Permanently</a>.
*/
