fn main() {
    let search_term = "picture";
    let quote = "\
Every face, every shp. bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the ssame with books.
What do we seek through millins of pages?";

    // linesが返すイテレータで、enumerate()に連鎖
    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
