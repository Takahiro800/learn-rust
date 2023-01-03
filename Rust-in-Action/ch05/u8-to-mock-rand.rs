fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000000000000000000;
    // nを32ビットに格納してから、左に15桁シフトして値を大きくする
    let large_n = (n as u32) << 15;

    // ビット演算のORによって基数のbaseと入力バイトをマージ
    let f32_bits = base | large_n;

    // u32型のf32_bitsを、f32として解釈する
    let m = f32::from_bits(f32_bits);

    // 出力の範囲を正規化
    2.0 * (m - 0.5)
}

fn main() {
    println!("入力範囲の最大値：　{:08b} -> {:?}", 0xff, mock_rand(0xff));
    println!("入力範囲の中央値：　{:08b} -> {:?}", 0x7f, mock_rand(0x7f));
    println!("入力範囲の最小値：　{:08b} -> {:?}", 0x00, mock_rand(0x00));
}
