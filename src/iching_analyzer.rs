//! Module containing functions for analyzing hexagrams and sequences of hexagrams.

use anyhow::{bail, Result};
use rand::seq::SliceRandom;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::collections::VecDeque;

use crate::iching::{create_hexagram, Hexagram, HexagramLine, Trigram, HEXAGRAMS};

/// The operations that can be applied to transform a hexagram.
#[derive(Clone, Debug, PartialEq)]
pub enum SearchOperation {
    /// No operation.
    NoOp,

    /// Inverse a single line.
    InverseLine(HexagramLine),

    /// Inverse the bottom trigram.
    InverseBottomTrigram,

    /// Inverse the top trigram.
    InverseTopTrigram,

    /// Reverse the order of the lines in the bottom trigram.
    ReverseBottomTrigram,

    /// Reverse the order of the lines in the top trigram.
    ReverseTopTrigram,

    /// Flip the bottom and top trigrams.
    FlipTrigrams,

    /// Mirror the bottom and top trigrams along the line that separates them.
    MirrorTrigrams,

    /// Create a new hexagram out of the bottom and top nuclear trigrams.
    NuclearTrigrams,

    /// Inverse all the lines in the hexagram.
    InverseHexagram,

    /// Reverse the order of the lines in the hexagram.
    ReverseHexagram,

    /// Create a new hexagram out of the bottom and top trigrams. The first line of the new hexagram
    /// is the first line of the bottom trigram. The second line of the new hexagram is the first
    /// line of the top trigram, and so on.
    MixTrigramsBottomFirst,

    /// Create a new hexagram out of the bottom and top trigrams. The first line of the new hexagram
    /// is the first line of the top trigram. The second line of the new hexagram is the first
    /// line of the bottom trigram, and so on.
    MixTrigramsTopFirst,
}

impl SearchOperation {
    /// Returns all possible search operations.
    fn all_operations() -> Vec<SearchOperation> {
        vec![
            Self::InverseLine(HexagramLine::First),
            Self::InverseLine(HexagramLine::Second),
            Self::InverseLine(HexagramLine::Third),
            Self::InverseLine(HexagramLine::Fourth),
            Self::InverseLine(HexagramLine::Fifth),
            Self::InverseLine(HexagramLine::Sixth),
            Self::InverseBottomTrigram,
            Self::InverseTopTrigram,
            Self::ReverseBottomTrigram,
            Self::ReverseTopTrigram,
            Self::FlipTrigrams,
            Self::MirrorTrigrams,
            Self::NuclearTrigrams,
            Self::InverseHexagram,
            Self::ReverseHexagram,
            Self::MixTrigramsBottomFirst,
            Self::MixTrigramsTopFirst,
        ]
    }

    /// Applies the search operation to the given hexagram.
    fn apply(&self, hexagram: &Hexagram) -> Hexagram {
        match self {
            Self::InverseLine(HexagramLine::First) => hexagram.inverse_line(HexagramLine::First),
            Self::InverseLine(HexagramLine::Second) => hexagram.inverse_line(HexagramLine::Second),
            Self::InverseLine(HexagramLine::Third) => hexagram.inverse_line(HexagramLine::Third),
            Self::InverseLine(HexagramLine::Fourth) => hexagram.inverse_line(HexagramLine::Fourth),
            Self::InverseLine(HexagramLine::Fifth) => hexagram.inverse_line(HexagramLine::Fifth),
            Self::InverseLine(HexagramLine::Sixth) => hexagram.inverse_line(HexagramLine::Sixth),
            Self::InverseHexagram => hexagram.inverse(),
            Self::InverseBottomTrigram => hexagram.inverse_bottom_trigram(),
            Self::InverseTopTrigram => hexagram.inverse_top_trigram(),
            Self::ReverseHexagram => hexagram.reverse(),
            Self::ReverseTopTrigram => hexagram.reverse_top_trigram(),
            Self::ReverseBottomTrigram => hexagram.reverse_bottom_trigram(),
            Self::FlipTrigrams => hexagram.flip_trigrams(),
            Self::MirrorTrigrams => hexagram.mirror_trigrams(),
            Self::NuclearTrigrams => hexagram.use_nuclear_trigrams(),
            Self::MixTrigramsBottomFirst => hexagram.mix_trigrams_bottom_first(),
            Self::MixTrigramsTopFirst => hexagram.mix_trigrams_top_first(),
            Self::NoOp => *hexagram,
        }
    }
}

/// The result of analyzing a hexagram.
pub struct HexagramAnalysis {
    /// The hexagram to analyze.
    pub hexagram: Hexagram,

    /// The bottom trigram of the hexagram.
    pub bottom_trigram: Trigram,

    /// The top trigram of the hexagram.
    pub top_trigram: Trigram,

    /// The bottom nuclear trigram of the hexagram, that is, the trigram formed by the second,
    /// third, and fourth lines.
    pub bottom_nuclear_trigram: Trigram,

    /// The top nuclear trigram of the hexagram, that is, the trigram formed by the third, fourth,
    /// and fifth lines.
    pub top_nuclear_trigram: Trigram,

    /// The list of hexagrams that can be reached from this hexagram by applying a single operation.
    pub reacheable_hexagrams: Vec<(Hexagram, SearchOperation)>,
}

impl HexagramAnalysis {
    /// Creates a new hexagram analysis.
    pub fn new(number: usize) -> Result<Self> {
        // Validate and create the hexagram
        if !(1..=64).contains(&number) {
            bail!("Invalid hexagram number: {}", number);
        }
        let lines = HEXAGRAMS[number - 1];
        let hexagram = create_hexagram(lines.0, lines.1);

        // Compute the information about the hexagram.
        let (bottom_trigram, top_trigram) = hexagram.trigrams();
        let (bottom_nuclear_trigram, top_nuclear_trigram) = hexagram.nuclear_trigrams();
        let reacheable_hexagrams = SearchOperation::all_operations()
            .into_iter()
            .map(|op| (op.apply(&hexagram), op))
            .filter(|(reacheable, _)| reacheable.number != hexagram.number)
            .collect();

        Ok(Self {
            hexagram,
            bottom_trigram,
            top_trigram,
            bottom_nuclear_trigram,
            top_nuclear_trigram,
            reacheable_hexagrams,
        })
    }

    /// Prints the hexagram analysis.
    pub fn print(&self) {
        println!(">>>>> Analysis of hexagram {}:", self.hexagram.number);
        println!();
        self.hexagram.print(None);
        println!();

        println!(">>> Bottom trigram:");
        println!();
        self.bottom_trigram.print();
        println!();

        println!(">>> Top trigram:");
        println!();
        self.top_trigram.print();
        println!();

        println!(">>> Bottom nuclear trigram:");
        println!();
        self.bottom_nuclear_trigram.print();
        println!();

        println!(">>> Top nuclear trigram:");
        println!();
        self.top_nuclear_trigram.print();
        println!();

        println!(">>> Reacheable hexagrams:");
        println!();
        for (hexagram, op) in &self.reacheable_hexagrams {
            println!(
                "> Hexagram {} can be reached by applying the operation {:?}",
                hexagram.number, op
            );
            println!();
            hexagram.print(None);
            println!();
        }
    }
}

/// A path between two hexagrams, containing the hexagrams and operations to transform them.
type Path = Vec<(Hexagram, SearchOperation)>;

/// Prints the shortest path between two hexagrams.
pub fn print_shortest_path(start: usize, end: usize, paths: &[Path]) {
    for (i, path) in paths.iter().enumerate() {
        println!(
            ">>> Path #{} from hexagram {} to hexagram {}:",
            i + 1,
            start,
            end
        );
        println!();

        for (i, (hexagram, op)) in path.iter().enumerate() {
            if i != 0 {
                println!(
                    "> Previous hexagram turns into hexagram {} by applying the operation {:?}",
                    hexagram.number, op
                );
                println!();
            }
            hexagram.print(None);
            println!();
        }
    }
}

/// Counts the total number of line changes in a path between two hexagrams.
pub fn count_line_changes(path: &Path) -> u64 {
    let mut count: u64 = 0;
    for i in 1..path.len() {
        count += path[i].0.num_line_changes(&path[i - 1].0) as u64;
    }
    count
}

/// Given two hexagrams, finds the shortest path between them.
pub struct HexagramSearcher {
    /// The initial hexagram from which to start the search.
    pub start_hexagram: Hexagram,

    /// The final hexagram to reach.
    pub end_hexagram: Hexagram,
}

impl HexagramSearcher {
    /// Creates a new hexagram searcher.
    pub fn new(start: usize, end: usize) -> Result<Self> {
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
        let start_hexagram = create_hexagram(start_lines.0, start_lines.1);
        let end_hexagram = create_hexagram(end_lines.0, end_lines.1);

        Ok(Self {
            start_hexagram,
            end_hexagram,
        })
    }

    /// Returns only the paths with the least line changes.
    fn find_least_lines_changed(paths: &[Path]) -> Vec<Path> {
        // Find the minimum number of lines for all the paths.
        let mut min_lines_changed = u64::MAX;
        for path in paths {
            let line_changes = count_line_changes(path);
            if min_lines_changed == u64::MAX || line_changes < min_lines_changed {
                min_lines_changed = line_changes;
            }
        }

        // Then, return all the paths with the minimum number of lines.
        let mut out = vec![];
        for path in paths {
            let line_changes = count_line_changes(path);
            if line_changes == min_lines_changed {
                out.push(path.clone());
            }
        }
        out
    }

    /// Returns all the shortest paths between the initial and final hexagrams.
    pub fn find_shortest_paths(&self, all: bool) -> Vec<Path> {
        // Create a queue of paths to search to perform a breadth-first search.
        let mut queue: VecDeque<Vec<(Hexagram, SearchOperation)>> = VecDeque::new();
        queue.push_back(vec![(self.start_hexagram, SearchOperation::NoOp)]);
        let ops = SearchOperation::all_operations();

        // Store a list of all the shortest paths. Then search until the queue is empty, or we find
        // all the shortest paths.
        let mut shortest_paths: Vec<Path> = vec![];
        while !queue.is_empty() {
            // Take the next path from the queue. Break out of the loop if a shorter path has been
            // found.
            let path = queue.pop_front().unwrap();
            if !shortest_paths.is_empty() && path.len() >= shortest_paths[0].len() {
                break;
            }

            // Try each operation on the current hexagram.
            let (current_hexagram, _) = path.last().unwrap();
            for operation in &ops {
                // Create a new hexagram with the given operation. Ignore it if it's already in the
                // path.
                let new_hexagram = operation.apply(current_hexagram);
                if path.iter().any(|(h, _)| h == &new_hexagram) {
                    continue;
                }

                // Create the new path and return it if it's the final hexagram. Otherwise, add the
                // new path to the queue.
                let mut new_path = path.clone();
                new_path.push((new_hexagram, operation.clone()));
                if new_hexagram == self.end_hexagram {
                    shortest_paths.push(new_path.clone());
                } else {
                    queue.push_back(new_path);
                }
            }
        }

        // Return the shortest paths. Either return all or only the ones with the least number of
        // lines changed.
        if all {
            shortest_paths
        } else {
            Self::find_least_lines_changed(&shortest_paths)
        }
    }
}

/// King Wen's sequence is the sequence of hexagrams as they appear in the I Ching.
pub fn king_wen() -> Vec<usize> {
    (1..=64).collect()
}

/// The result of performing a sequence analysis.
#[derive(Clone, Debug, Default)]
pub struct SequenceAnalysis {
    /// The sequence of hexagrams.
    pub sequence: Vec<usize>,

    /// The shortest paths between each pair of hexagrams.
    pub shortest_paths: Vec<Vec<Path>>,

    /// The total number of operations between the initial and final hexagrams in the sequence.
    pub total_ops: u64,

    /// The total number of line changes between the initial and final hexagrams in the sequence.
    pub total_line_changes: u64,

    /// The total number of paths from the initial to the final hexagram.
    pub total_paths: u128,
}

impl SequenceAnalysis {
    /// Prints the info in the analysis minus the paths themselves.
    fn print_info(&self) {
        println!(">>> Sequence of hexagrams: {:?}", self.sequence);
        println!(">>> Total operations: {}", self.total_ops);
        println!(">>> Total line changes: {}", self.total_line_changes);
        println!(
            ">>> Lines changed per operation: {0:.3}",
            self.total_line_changes as f32 / self.total_ops as f32
        );
        println!(">>> Total paths: {}", self.total_paths);
        println!();
    }

    /// Prints the entire analysis.
    pub fn print(&self) {
        // Print the part of the analysis concerning the whole sequence.
        println!(">>>>> Analysis of sequence of hexagrams");
        println!();
        self.print_info();

        // Print all the shortest paths between each pair of hexagrams.
        println!(">>> Shortest paths between each pair of hexagrams:");
        println!();
        for i in 1..self.sequence.len() {
            print_shortest_path(
                self.sequence[i - 1],
                self.sequence[i],
                &self.shortest_paths[i - 1],
            );
        }
    }

    /// Prints a comparison between this analysis and another one.
    pub fn print_comparison(&self, other: &Self) {
        println!(">>>>> Comparison of sequence analyses");
        println!();
        self.print_info();
        other.print_info();
    }

    /// Produces the analysis of the sequence of hexagrams.
    pub fn new(sequence: Vec<usize>) -> Result<Self> {
        // Find the shortest paths between each pair of hexagrams.
        let mut shortest_paths = vec![];
        for i in 1..sequence.len() {
            let searcher = HexagramSearcher::new(sequence[i - 1], sequence[i])?;
            let paths = searcher.find_shortest_paths(false);
            shortest_paths.push(paths);
        }

        // Compute the other values from the shortest paths.
        let total_ops = shortest_paths
            .iter()
            .map(|paths| (paths[0].len() - 1) as u64)
            .sum();
        let total_line_changes = shortest_paths
            .iter()
            .map(|paths| count_line_changes(&paths[0]))
            .sum();
        let total_paths = shortest_paths
            .iter()
            .map(|paths| paths.len() as u128)
            .product();

        Ok(Self {
            sequence,
            shortest_paths,
            total_ops,
            total_line_changes,
            total_paths,
        })
    }
}

/// Finds the best random shuffling of the King Wen's sequence by the number of operations.
pub fn find_min_random_sequence(num_sequences: usize) -> Result<SequenceAnalysis> {
    Ok((0..num_sequences)
        .into_par_iter()
        .map(|_| {
            let mut random_sequence = king_wen();
            random_sequence.shuffle(&mut rand::thread_rng());
            SequenceAnalysis::new(random_sequence)
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .min_by_key(|analysis| analysis.total_ops)
        .unwrap()
        .clone())
}

#[cfg(test)]
mod test {
    use crate::{
        iching::{create_hexagram, HEXAGRAMS},
        iching_analyzer::SearchOperation,
    };

    use super::HexagramSearcher;

    #[test]
    fn test_find_path() {
        let searcher = HexagramSearcher {
            start_hexagram: create_hexagram(1, HEXAGRAMS[0].1),
            end_hexagram: create_hexagram(2, HEXAGRAMS[1].1),
        };
        let expected_path = vec![vec![
            (create_hexagram(1, HEXAGRAMS[0].1), SearchOperation::NoOp),
            (
                create_hexagram(2, HEXAGRAMS[1].1),
                SearchOperation::InverseHexagram,
            ),
        ]];
        let path = searcher.find_shortest_paths(false);
        assert_eq!(path, expected_path);
    }
}
