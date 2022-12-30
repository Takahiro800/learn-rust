use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("docs.md").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line).unwrap();

        if len == 0 {
            break;
        }

        println!("{} ({} bytes long)", line, len);

        // Stringの長さを０に縮めて、前の行の内容が残ってしまうのを防ぐ
        line.truncate(0);
    }
}

/*
NOTE
- Fileオブジェクトの作成には、パス引数とファイルが存在しない時のエラー処理が必要。
*/
