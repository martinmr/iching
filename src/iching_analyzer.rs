use std::collections::VecDeque;

use crate::iching::{Hexagram, HexagramLine};

#[derive(Clone, Debug, PartialEq)]
enum SearchOperation {
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
            Self::NoOp => hexagram.clone(),
        }
    }
}

/// Given two hexagrams, finds the shortest path between them.
struct HexagramSearcher {
    initial_hexagram: Hexagram,
    final_hexagram: Hexagram,
}

impl HexagramSearcher {
    /// Returns the shortest path between the initial and final hexagrams.
    fn find_path(&self) -> Vec<(Hexagram, SearchOperation)> {
        // Create a queue of paths to search to perform a breadth-first search.
        let mut queue: VecDeque<Vec<(Hexagram, SearchOperation)>> = VecDeque::new();
        queue.push_back(vec![(self.initial_hexagram.clone(), SearchOperation::NoOp)]);
        let ops = SearchOperation::all_operations();

        while !queue.is_empty() {
            let path = queue.pop_front().unwrap();
            let (current_hexagram, _) = path.last().unwrap();

            for operation in &ops {
                // Create a new hexagram with the given operation. Ignore it if it's already in the
                // path.
                let new_hexagram = operation.apply(current_hexagram);
                if path.iter().any(|(h, _)| h == &new_hexagram) {
                    continue;
                }

                // Create the new path and return it if it's the final hexagram. Otherwise add the
                // new path to the queue.
                let mut new_path = path.clone();
                new_path.push((new_hexagram, operation.clone()));
                if new_hexagram == self.final_hexagram {
                    return new_path;
                }
                queue.push_back(new_path);
            }
        }

        // This point should never be reached, but return an empty path to satisfy the compiler.
        vec![]
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
        let expected_path = vec![
            (create_hexagram(1, HEXAGRAMS[0].1), SearchOperation::NoOp),
            (
                create_hexagram(2, HEXAGRAMS[1].1),
                SearchOperation::InverseHexagram,
            ),
        ];
        let path = searcher.find_path();
        assert_eq!(path, expected_path);
    }
}
