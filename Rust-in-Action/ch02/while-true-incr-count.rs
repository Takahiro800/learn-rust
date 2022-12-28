use std::time::{Duration, Instant};

fn main() {
    let mut count = 1;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now(); // システムクロックから開始時刻Instantを取得

    // 2つのInstantの減算は Durationを返す
    while (Instant::now() - start) < time_limit {
        count += 1;
    }

    println!("{}", count);
}
