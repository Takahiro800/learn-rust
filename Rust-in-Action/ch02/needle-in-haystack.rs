fn main() {
    let needle = 0o204;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }
}

/* 実行結果
132
 */
