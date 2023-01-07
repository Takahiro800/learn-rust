use std::mem::drop;

fn main() {
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    let result1 = *a + *b + *c;
    println!("a: {:p}", a);

    // drop()を呼び出して、メモリを開放する
    drop(a);
    let d = Box::new(1);
    let result2 = *b + *c + *d;

    println!("{} {}", result1, result2);
    println!("b: {:p}", b);
    println!("c: {:p}", c);
    println!("d: {:p}", d);
}
