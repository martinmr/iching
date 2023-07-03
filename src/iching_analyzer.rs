use anyhow::{bail, Result};
use std::collections::VecDeque;

use crate::iching::{create_hexagram, Hexagram, HexagramLine, HEXAGRAMS};

#[derive(Clone, Debug, PartialEq)]
pub enum SearchOperation {
    NoOp,
    InverseLine(HexagramLine),
    InverseBottomTrigram,
    InverseTopTrigram,
    ReverseBottomTrigram,
    ReverseTopTrigram,
    MirrorTrigrams,
    InverseHexagram,
    ReverseHexagram,
}

impl SearchOperation {
    /// Returns all possible search operations in the order they should be applied.
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
            Self::MirrorTrigrams,
            Self::InverseHexagram,
            Self::ReverseHexagram,
        ]
    }

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
            Self::MirrorTrigrams => hexagram.mirror_trigrams(),
            Self::NoOp => *hexagram,
        }
    }
}

/// A path between two hexagrams, containing the hexagrams and operations to transform them.
type Path = Vec<(Hexagram, SearchOperation)>;

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

pub fn count_line_changes(path: &Path) -> u64 {
    let mut count: u64 = 0;
    for i in 1..path.len() {
        count += path[i].0.num_line_changes(&path[i - 1].0) as u64;
    }
    count
}

/// Given two hexagrams, finds the shortest path between them.
pub struct HexagramSearcher {
    pub initial_hexagram: Hexagram,
    pub final_hexagram: Hexagram,
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
        let initial_hexagram = create_hexagram(start_lines.0, start_lines.1);
        let final_hexagram = create_hexagram(end_lines.0, end_lines.1);

        Ok(Self {
            initial_hexagram,
            final_hexagram,
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
        queue.push_back(vec![(self.initial_hexagram, SearchOperation::NoOp)]);
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
                if new_hexagram == self.final_hexagram {
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

// King Wen's sequence is the sequence of hexagrams as they appear in the I Ching.
pub fn king_wen() -> Vec<usize> {
    (1..=64).collect()
}

/// The result of performing a sequence analysis.
pub struct SequenceAnalysis {
    /// The sequence of hexagrams.
    pub sequence: Vec<usize>,

    /// The shortest paths between each pair of hexagrams.
    pub shortest_paths: Vec<Vec<Path>>,

    /// The total number of operations between the initial and final hexagrams in the sequence.
    pub total_ops: u64,

    /// The total number of line changes between the initial and final hexagrams in the sequence.
    pub total_line_changes: u64,
}

impl SequenceAnalysis {
    /// Prints the entire analysis.
    pub fn print(&self) {
        // Print the part of the analysis concerning the whole sequence.
        println!(">>>>>> Analysis of sequence of hexagrams");
        println!();
        println!(">>> Sequence of hexagrams: {:?}", self.sequence);
        println!(">>> Total operations: {}", self.total_ops);
        println!(">>> Total line changes: {}", self.total_line_changes);
        println!();

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
        println!(">>>>>> Comparison of sequence analyses");
        println!();
        println!(">>> Sequence of hexagrams: {:?}", self.sequence);
        println!(">>> Total operations: {}", self.total_ops);
        println!(">>> Total line changes: {}", self.total_line_changes);
        println!();
        println!(">>> Sequence of hexagrams: {:?}", other.sequence);
        println!(">>> Total operations: {}", other.total_ops);
        println!(">>> Total line changes: {}", other.total_line_changes);
        println!();
    }
}

/// Analyzes a sequence of hexagrams.
pub struct SequenceAnalyzer {
    /// The sequence of hexagrams to analyze, as a vector of hexagram numbers.
    pub sequence: Vec<usize>,
}

impl SequenceAnalyzer {
    /// Produces the analysis of the sequence of hexagrams.
    pub fn analyze(&self) -> SequenceAnalysis {
        // Find the shortest paths between each pair of hexagrams.
        let mut shortest_paths = vec![];
        for i in 1..self.sequence.len() {
            let searcher = HexagramSearcher::new(self.sequence[i - 1], self.sequence[i]).unwrap();
            let paths = searcher.find_shortest_paths(false);
            shortest_paths.push(paths);
        }

        // Compute the sum of operations and line changes.
        let total_ops = shortest_paths
            .iter()
            .map(|paths| paths[0].len() as u64)
            .sum();
        let total_line_changes = shortest_paths
            .iter()
            .map(|paths| count_line_changes(&paths[0]))
            .sum();

        SequenceAnalysis {
            sequence: self.sequence.clone(),
            shortest_paths,
            total_ops,
            total_line_changes,
        }
    }
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
            initial_hexagram: create_hexagram(1, HEXAGRAMS[0].1),
            final_hexagram: create_hexagram(2, HEXAGRAMS[1].1),
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
