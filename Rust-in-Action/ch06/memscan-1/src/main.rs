fn main() {
    let mut n_nonzero = 0;

    // iが0の時、ptrをでリファレンスするのはアクセス違反で実行不可能。
    // 生ポインタのデリファレンスをunsafeの中ででしかできない理由でもある。
    for i in 1..10000 {
        // NULLポインタによる例外を防ぐ
        let ptr = i as *const u8;
        let byte_at_addr = unsafe { *ptr };

        if byte_at_addr != 0 {
            n_nonzero += 1;
        }
    }

    println!("non_zero bytes in memory: {}", n_nonzero);
}
