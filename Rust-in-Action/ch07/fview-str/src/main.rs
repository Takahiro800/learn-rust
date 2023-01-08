use std::io::*;

const BYTES_PER_LINE: usize = 16;
const INPUT: &'static [u8] = br#"
fn main(){
    println!("Hello, world!);
}"#;

fn main() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = vec![]; // 入力用に内部バッファのスペースを確保
    INPUT.read_to_end(&mut buffer)?; // 入力を読んで、内部バッファに入れる

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}] ", position_in_input);
        for byte in line {
            print!("[{:02x}] ", byte);
        }
        println!();
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}
