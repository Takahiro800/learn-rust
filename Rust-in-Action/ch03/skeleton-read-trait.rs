#![allow(unused_variables)]

#[derive(Debug)]
struct File;

trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}
/* NOTE
トレイトのブロックは、関数実装が従うべき型シグネチャを含む。
擬似型のSelf は、いつかReadを実装する型で置き換えられるプレースホルダー
*/

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0)
        // 必要な型シグネチャに従う、単純なスタブの値。
    }
}

fn main() {
    let f = File {};
    let mut buffer = vec![];
    let n_bytes = f.read(&mut buffer).unwrap();
    println!("{} byte(s) read from {:?}", n_bytes, f);
}
