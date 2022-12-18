fn main() {
    // needleは、heystackに含まれる値を持つ変数
    // let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        // このmatch式は、変数に結び付けることが可能な値を返す
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }
}

/* 実行結果
42: hit!
132: hit!
*/
