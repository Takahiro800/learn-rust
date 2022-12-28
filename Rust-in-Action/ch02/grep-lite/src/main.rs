use clap::{App, Arg};
use regex::Regex;

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searche for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

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

/*
documentを展開する
./target/debug/grep-lite --help
*/
