use std::error::Error;

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Key Youens-Clark <kyclark@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .multiple(true)
                .help("Input file(s)")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("bytes")
                .long("bytes")
                .short("c")
                .help("Show byte count"),
        )
        .arg(
            Arg::with_name("chars")
                .long("chars")
                .short("m")
                .help("Show character count"),
        )
        .arg(
            Arg::with_name("lines")
                .long("lines")
                .short("l")
                .help("Show line count"),
        )
        .arg(
            Arg::with_name("words")
                .long("words")
                .short("w")
                .help("Show word count"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: matches.is_present("lines"),
        words: matches.is_present("words"),
        bytes: matches.is_present("bytes"),
        chars: matches.is_present("chars"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
