// DONE ファイルを表現する永続的なオブジェクトを作る
// DONE read()を実装する。読み込みの失敗を実装する
// TODO open(): OSが返すエラ〜メッセージを格納する方法
// TODO close(): OSが返すエラ〜メッセージを格納する方法
// TODO 関数をメソッドにする。(open(f)よりもf.open()が望ましい。)

#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn open(f: &mut File) -> bool {
    true
}
fn close(f: &mut File) -> bool {
    true
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

    // Vecに対して、十分な容量を確保する。既に十分な場合は、何もしない。
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
}

fn main() {
    let mut f2 = File {
        name: String::from("2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);
    let f2_length = read(&f2, &mut buffer);
    close(&mut f2);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} byptes long", &f2.name, &f2_length);
    println!("{}", text);
}
