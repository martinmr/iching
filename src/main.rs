//! CLI utility for generating I Ching readings and analyzing hexagrams.

pub mod iching;
pub mod iching_analyzer;

use anyhow::Result;
use clap::{Parser, Subcommand};
use iching_analyzer::{
    find_min_random_sequence, king_wen, print_shortest_path, HexagramAnalysis, HexagramSearcher,
    SequenceAnalyzer,
};

use crate::iching::{RandomnessMode, ReadingMethod};

/// Contains subcommands used for manipulating git repositories containing Trane courses.
#[derive(Clone, Debug, Subcommand)]
enum AnalyzeSubcommand {
    #[clap(about = "Compare a random sequence to King Wen's sequence")]
    CompareKingWen {
        #[clap(
            help = "The number of random sequences to generate. Only the one with the least \
            operations will be printed"
        )]
        #[clap(default_value = "1")]
        #[clap(short, long)]
        num_sequences: usize,
    },

    #[clap(about = "Print an analysis of the given hexagram")]
    Hexagram {
        #[clap(help = "The hexagram to analyze")]
        number: usize,
    },

    #[clap(about = "Print an analysis of King Wen's sequence")]
    KingWen,

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
                IChingSubcommand::Analyze(AnalyzeSubcommand::CompareKingWen { num_sequences }) => {
                    // Generate King Wen's sequence and analysis.
                    let king_wen_sequence = king_wen();
                    let king_wen_analysis = SequenceAnalyzer {
                        sequence: king_wen_sequence,
                    }
                    .analyze();

                    // Generate random sequences and analyze them.
                    let min_sequence = find_min_random_sequence(num_sequences);
                    king_wen_analysis.print_comparison(&min_sequence);
                }
                IChingSubcommand::Analyze(AnalyzeSubcommand::Hexagram { number }) => {
                    let analysis = HexagramAnalysis::new(number)?;
                    analysis.print();
                }
                IChingSubcommand::Analyze(AnalyzeSubcommand::KingWen) => {
                    let analyzer = SequenceAnalyzer {
                        sequence: king_wen(),
                    };
                    analyzer.analyze().print();
                }
                IChingSubcommand::Analyze(AnalyzeSubcommand::ShortestDistance {
                    start,
                    end,
                    all,
                }) => {
                    // Perform the search.
                    let searcher = HexagramSearcher::new(start, end)?;
                    let paths = searcher.find_shortest_paths(all);

                    // Print all the paths
                    println!(">>>>> Shortest path search from {} to {}", start, end);
                    println!();
                    println!(">>> Shortest path search found {} path(s)", paths.len());
                    println!();
                    print_shortest_path(start, end, &paths)
                }
            }
        }
    }
    Ok(())
}
