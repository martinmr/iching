mod iching;
mod iching_analyzer;

use anyhow::Result;
use clap::{Parser, Subcommand};
use iching_analyzer::{print_shortest_path, HexagramSearcher, SequenceAnalyzer};

use crate::iching::{RandomnessMode, ReadingMethod};

/// Contains subcommands used for manipulating git repositories containing Trane courses.
#[derive(Clone, Debug, Subcommand)]
enum AnalyzeSubcommand {
    #[clap(about = "Find the shortest path between two hexagrams")]
    ShortestDistance {
        #[clap(help = "The hexagram from which to start")]
        start: usize,

        #[clap(help = "The hexagram to reach")]
        end: usize,

        #[clap(help = "Print all shortest paths instead of the ones with the least line changes")]
        #[clap(short, long)]
        #[clap(default_value = "false")]
        all: bool,
    },

    #[clap(about = "Print an analysis of King Wen's sequence")]
    KingWen,
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
                IChingSubcommand::Analyze(AnalyzeSubcommand::ShortestDistance {
                    start,
                    end,
                    all,
                }) => {
                    // Perform the search.
                    let searcher = HexagramSearcher::new(start, end)?;
                    let paths = searcher.find_shortest_paths(all);

                    // Print all the paths
                    println!(">>> Shortest path search found {} path(s)", paths.len());
                    println!();
                    print_shortest_path(start, end, &paths)
                }
                IChingSubcommand::Analyze(AnalyzeSubcommand::KingWen) => {
                    // King Wen's sequence is the sequence of hexagrams as they appear in the
                    // I Ching.
                    let king_wen: Vec<usize> = (1..=64).collect();
                    let analyzer = SequenceAnalyzer { sequence: king_wen };
                    analyzer.analyze().print();
                }
            }
        }
    }
    Ok(())
}
