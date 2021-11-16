mod error;
mod iching;

use clap::{App, Arg};

fn main() -> Result<(), error::Error> {
    let matches = App::new("I Ching")
        .version("0.1.0")
        .about("CLI program to generate an I Ching reading based on a random drawing")
        .arg(
            Arg::with_name("mode")
                .long("mode")
                .value_name("MODE")
                .default_value("random")
                .help("Mode used to generate the reading. Takes a value of either 'random' or 'pseudorandom'")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("question")
                .long("question")
                .help("The question to ask to the I Ching")
                .default_value("")
                .required(false)
        )
        .get_matches();

    let mode = iching::Mode::from(matches.value_of("mode").unwrap_or("random"));
    let question = matches.value_of("question").unwrap_or("");

    let result = iching::generate_reading(mode, question)?;
    result.print();
    Ok(())
}
