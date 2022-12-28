use regex::Regex;

fn main() {
    let re = Regex::new("picture").unwrap();

    let quote = "\
Every face, every shp. bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the ssame with books.  What do we seek through millins of pages?";

    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (), // nullのプレースホルダー値
        }
    }
}
