// ファイルを開きその内容を反復処理する

use std::env;
use std::fs::File;
use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;

fn main() {
    // cargo run -- Cargo.tomlの場合、nth(0)は以下になる
    // target/debug/fview
    let arg1 = env::args().nth(1);

    let fname = arg1.expect("usage: fview FILENAME");

    let mut f = File::open(&fname).expect("Unable to open file.");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[0x{:08x}] ", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(". "),
                0xff => print!("## "),
                _ => print!("[{:02x}] ", byte),
            }
        }

        println!("");
        pos += BYTES_PER_LINE;
    }
}
/*  cargo run -- Cargo.toml
[0x00000000] [5b] [70] [61] [63] [6b] [61] [67] [65] [5d] [0a] [6e] [61] [6d] [65] [20] [3d]
[0x00000010] [20] [22] [66] [76] [69] [65] [77] [22] [0a] [76] [65] [72] [73] [69] [6f] [6e]
[0x00000020] [20] [3d] [20] [22] [30] [2e] [31] [2e] [30] [22] [0a] [65] [64] [69] [74] [69]
[0x00000030] [6f] [6e] [20] [3d] [20] [22] [32] [30] [32] [31] [22] [0a] [0a] [23] [20] [53]
[0x00000040] [65] [65] [20] [6d] [6f] [72] [65] [20] [6b] [65] [79] [73] [20] [61] [6e] [64]
[0x00000050] [20] [74] [68] [65] [69] [72] [20] [64] [65] [66] [69] [6e] [69] [74] [69] [6f]
[0x00000060] [6e] [73] [20] [61] [74] [20] [68] [74] [74] [70] [73] [3a] [2f] [2f] [64] [6f]
[0x00000070] [63] [2e] [72] [75] [73] [74] [2d] [6c] [61] [6e] [67] [2e] [6f] [72] [67] [2f]
[0x00000080] [63] [61] [72] [67] [6f] [2f] [72] [65] [66] [65] [72] [65] [6e] [63] [65] [2f]
[0x00000090] [6d] [61] [6e] [69] [66] [65] [73] [74] [2e] [68] [74] [6d] [6c] [0a] [0a] [5b]
 */
