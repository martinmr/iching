mod error;
mod iching;

use crate::iching::{RandomnessMode, ReadingMethod};
use clap::Parser;

/// Arguments for the CLI.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The method used to generate the reading.
    #[arg(long, default_value_t = ReadingMethod::YarrowStalks)]
    method: ReadingMethod,

    /// Whether to use random.org or a pseudorandom number generator to generate the reading.
    #[arg(long, default_value_t = RandomnessMode::Random)]
    randomness: RandomnessMode,

    /// The optional question to ask the I Ching.
    #[arg(short, long, default_value = "")]
    question: String,
}

fn main() -> Result<(), error::Error> {
    let args = Args::parse();
    let result = iching::generate_reading(args.method, args.randomness, &args.question)?;
    result.print();
    Ok(())
}
