// Hostname がnew type
struct Hostname(String);

fn connect(host: Hostname) {
    // 数値インデックスを使って、根底にあるデータにアクセスする
    println!("connected to {}", host.0);
}

fn main() {
    let ordinary_string = String::from("localhost");

    // Hostnameを期待する関数に、普通の文字列を渡す
    let host = Hostname(ordinary_string.close());

    connect(ordinary_string);
}
