use std::collections::VecDeque;

use crate::iching::{Hexagram, HexagramLine};

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

/// Given two hexagrams, finds the shortest path between them.
pub struct HexagramSearcher {
    pub initial_hexagram: Hexagram,
    pub final_hexagram: Hexagram,
}

/// A path between two hexagrams, containing the hexagrams and operations to transform them.
type Path = Vec<(Hexagram, SearchOperation)>;

impl HexagramSearcher {
    /// Returns only the paths with the least line changes.
    fn find_least_lines_changed(paths: &[Path]) -> Vec<Path> {
        // Find the minimum number of lines for all the paths.
        let mut min_lines_changed = usize::MAX;
        for path in paths {
            let mut lines_changed = 0;
            for i in 1..path.len() {
                lines_changed += path[i].0.num_line_changes(&path[i - 1].0);
            }
            if min_lines_changed == usize::MAX || lines_changed < min_lines_changed {
                min_lines_changed = lines_changed;
            }
        }

        // Then, return all the paths with the minimum number of lines.
        let mut out = vec![];
        for path in paths {
            let mut lines_changed = 0;
            for i in 1..path.len() {
                lines_changed += path[i].0.num_line_changes(&path[i - 1].0);
            }
            if lines_changed == min_lines_changed {
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
