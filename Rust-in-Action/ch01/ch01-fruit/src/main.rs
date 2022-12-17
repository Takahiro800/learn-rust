/* バッファオーバーフローのせいでパニックとなる例 */
fn main() {
    let fruit = vec!['🥝', '🍌', '🍇'];

    let buffer_overflow = fruit[4];
    assert_eq!(buffer_overflow, '🍉')
}
