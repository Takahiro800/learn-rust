#![allow(unused_variables)] // コンパイラの警告を緩和

type File = String;

fn open(f: &mut File) -> bool {
    true
}
fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!() // 到達したらプログラムをクラッシュさせる
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(f1, vec![]);
    close(&mut f1);
}

// TODO ファイルを表現する永続的なオブジェクトを作る
// TODO read()を実装する。読み込みの失敗を実装する
// TODO open(): OSが返すエラ〜メッセージを格納する方法
// TODO close(): OSが返すエラ〜メッセージを格納する方法
// TODO 関数をメソッドにする。(open(f)よりもf.open()が望ましい。)
