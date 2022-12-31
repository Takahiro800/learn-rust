use rand::random;

static mut ERROR: isize = 0;

struct File;

#[allow(unused_variables)]
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() {
        // 8回に1回trueを返す
        unsafe {
            ERROR = 1;
        }
    }
    0 //読むのは常に０バイト
}

fn main() {
    let f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }
}
