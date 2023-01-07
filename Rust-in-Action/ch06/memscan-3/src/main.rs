// global staticの作成
static GLOBAL: i32 = 10000;

// 何もしないnoop()の中にローカル変数を作るのは、main()の外の何かにメモリアドレスを持たせたいから
fn noop() -> *const i32 {
    let noop_local = 12345;
    &noop_local as *const i32
}

fn main() {
    let local_str = "a";
    let local_int = 123;
    let boxed_str = Box::new('b');
    let boxed_int = Box::new(789);
    let fn_int = noop();

    println!("GLOBAL {:p}", &GLOBAL as *const i32);
    println!("local_str {:p}", local_str as *const str);
    println!("local_int {:p}", &local_int as *const i32);
    println!("boxed_str {:p}", Box::into_raw(boxed_str));
    println!("boxed_int {:p}", Box::into_raw(boxed_int));
    println!("fn_int {:p}", fn_int);
}

/* Note
GLOBAL    0x10034e69c
local_str 0x10034e6a0
local_int 0x16fae646c
boxed_str 0x11fe06a10
boxed_int 0x11fe06a20
fn_int    0x16fae63bc
*/
