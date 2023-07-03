mod iching;
mod iching_analyzer;

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use iching::{create_hexagram, HEXAGRAMS};
use iching_analyzer::HexagramSearcher;

use crate::iching::{RandomnessMode, ReadingMethod};

/// Contains subcommands used for manipulating git repositories containing Trane courses.
#[derive(Clone, Debug, Subcommand)]
enum AnalyzeSubcommand {
    ShortestDistance {
        #[clap(help = "The hexagram from which to start")]
        start: usize,

        #[clap(help = "The hexagram to reach")]
        end: usize,
    },
}

/// Sub-commands for the CLI.
#[derive(Clone, Debug, Subcommand)]
enum IChingSubcommand {
    #[clap(about = "Sub-commands to analyze hexagrams")]
    #[clap(subcommand)]
    Analyze(AnalyzeSubcommand),
}

/// Arguments for the CLI.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The method used to generate the reading.
    #[arg(short, long, default_value_t = ReadingMethod::YarrowStalks)]
    method: ReadingMethod,

    /// Whether to use random.org or a pseudo-random number generator to generate the reading.
    #[arg(short, long, default_value_t = RandomnessMode::Random)]
    randomness: RandomnessMode,

    /// The optional question to ask the I Ching.
    #[arg(short, long, default_value = "")]
    question: String,

    #[clap(subcommand)]
    subcommand: Option<IChingSubcommand>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.subcommand {
        None => {
            let result = iching::generate_reading(args.method, args.randomness, &args.question)?;
            result.print();
        }
        Some(subcommand) => {
            match subcommand {
                IChingSubcommand::Analyze(AnalyzeSubcommand::ShortestDistance { start, end }) => {
                    // Validate the hexagram numbers.
                    if !(1..=64).contains(&start) {
                        bail!("Invalid start hexagram number: {}", start);
                    }
                    if !(1..=64).contains(&end) {
                        bail!("Invalid end hexagram number: {}", end);
                    }

                    // Get the lines and hexagrams.
                    let start_lines = HEXAGRAMS[start - 1];
                    let end_lines = HEXAGRAMS[end - 1];
                    let initial_hexagram = create_hexagram(start_lines.0, start_lines.1);
                    let final_hexagram = create_hexagram(end_lines.0, end_lines.1);

                    // Perform the search.
                    let searcher = HexagramSearcher {
                        initial_hexagram,
                        final_hexagram,
                    };
                    let path = searcher.find_path();

                    // Print the path.
                    for (i, (hexagram, op)) in path.iter().enumerate() {
                        if i != 0 {
                            println!("Previous hexagram turns into hexagram {} by applying the operation {:?}", hexagram.number, op);
                            println!();
                        }
                        hexagram.print(None);
                        println!();
                    }
                }
            }
        }
    }
    Ok(())
}
