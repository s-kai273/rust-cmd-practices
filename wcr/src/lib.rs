use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

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

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Key Youens-Clark <kyclark@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .multiple(true)
                .help("Input file(s)")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("bytes")
                .long("bytes")
                .short("c")
                .help("Show byte count")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("chars")
                .long("chars")
                .short("m")
                .help("Show character count")
                .takes_value(false)
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::with_name("lines")
                .long("lines")
                .short("l")
                .help("Show line count")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("words")
                .long("words")
                .short("w")
                .help("Show word count")
                .takes_value(false),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines,
        words: words,
        bytes: bytes,
        chars: chars,
    })
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{} : {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{:>8}{:>8}{:>8} {}",
                        info.num_lines, info.num_words, info.num_bytes, filename
                    );
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{FileInfo, count};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
