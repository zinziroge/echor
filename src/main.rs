use clap::{App, Arg};

fn main() {
    //println!("Hello, world!");
    //println!("{:?}", std::env::args());
    let matches = App::new("echor")
        .version("0.1.0")
        .author("shin")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    println!("{:#?}", matches);

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    let mut ending = "\n";
    println!("{}", omit_newline);
    if omit_newline {
        ending = "";
    }
    print!("{}{}", text.join(" "), ending);

}
