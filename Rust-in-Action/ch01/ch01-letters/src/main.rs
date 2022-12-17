/* イテレータの反復処理中に、イテレータそのものを変更しようとする */
fn main() {
    let mut letters = vec!["a", "b", "c"];

    for letter in letters {
        println!("{}", letter);
        letters.push(letter.clone()); // 個々のletterをコピーして末尾に追加
        letters.push(letter.clone());
    }
}
