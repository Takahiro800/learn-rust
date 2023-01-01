use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64,
}

fn main() {
    let base: Rc<RefCell<GroundStation>> =
        Rc::new(RefCell::new(GroundStation { radio_freq: 87.65 }));
    println!("base: {:?}", base);

    // 新しいスコープを導入。その中ではbaseの可変借用が許される
    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base_2: {:?}", base_2);
    }

    println!("base: {:?}", base);

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;

    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);
}

/* Note
- 他の型でラッピングすることで、型に機能を盛り込むと、実行時の性能が落ちる
-　Cloneの実装に許容できないほどコストがかかる場合は、Rc<T>が手軽な代案かもしれない。これによって、２つの場所から所有権を共有できるようになる。

- Rc<T>は「スレッド安全」ではない。
マルチスレッドのコードでは、Rc<T>の代わりにArc<T>を、Rc<RefCell<T>>の代わりにArc<Mutex<T>>を使う方が、ずっと優れている。
Arcは、Atomic reference counterの略
 */
