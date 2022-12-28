use std::thread;

fn main() {
    let mut data = 100;
    // thread::spawn()は引数としてクロージャを受け取る
    thread::spawn(|| {
        data = 500;
    });
    thread::spawn(|| {
        data = 1000;
    });
    println!("{}", data);
}

/*
スレッド(thread)：詳細を知りたい場合は『スレッドを使用してコードを同時に走らせる』(https://doc.rust-jp.rs/book-ja/ch16-01-threads.html)
thread::spqan()の呼び出しで、２本のスレッドを生成
それぞれ、引数として１個のクロージャ(closure)を受け取る.
*/
