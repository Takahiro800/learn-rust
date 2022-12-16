fn greet_world() {
    println!("Hello World!");
    let shouthern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [shouthern_germany, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
