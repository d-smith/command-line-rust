use std::error::Error;
use clap::{App,Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("d-smith")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-")
        )   
        .arg(
            Arg::with_name("number-blank")
                .short("b")
                .help("number non-empty output lines, overrides -n")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .help("number all output lines")
                .takes_value(false)
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-blank"),
    })
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}