fn main() {
    let search_term = "picture";
    let quote = "\
Every face, every shp. bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the ssame with books.
What do we seek through millins of pages?";

    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}", line);
        }
    }
}

/*
String型　：　他の言語の文字列型に近い
str型　：　高性能だが機能に乏しい。通常 &strの形で使う
*/
