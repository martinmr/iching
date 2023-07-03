use anyhow::{anyhow, bail, Result};
use clap::ValueEnum;
use lazy_static::lazy_static;
use rand::Rng;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

/// The type of line in a hexagram.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Line {
    /// An open line, representing yin energy.
    Open,

    /// A closed line, representing yang energy.
    Closed,
}

impl Line {
    /// Inverses the line to the opposite value.
    pub fn inverse(&self) -> Line {
        match self {
            Line::Open => Line::Closed,
            Line::Closed => Line::Open,
        }
    }
}

impl From<u8> for Line {
    fn from(n: u8) -> Self {
        match n {
            0 => Line::Open,
            _ => Line::Closed,
        }
    }
}

/// The position of a line in a trigram.
#[allow(dead_code)]
pub enum TrigramLine {
    First,
    Second,
    Third,
}

impl TrigramLine {
    /// Converts a line into an array index.
    #[allow(dead_code)]
    fn line_to_index(&self) -> usize {
        match self {
            TrigramLine::First => 0,
            TrigramLine::Second => 1,
            TrigramLine::Third => 2,
        }
    }
}

/// A single trigram, consisting of three lines.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Trigram {
    /// The number of the trigram, from 1 to 8.
    pub number: u8,

    /// The lines of the hexagram. The first line is the bottom line, and the last line is the top
    /// one.
    pub lines: [Line; 3],
}

impl Trigram {
    /// Prints the trigram to the console.
    #[allow(dead_code)]
    pub fn print(&self) {
        println!("     {}\n", self.number);
        for line in self.lines.iter().rev() {
            match line {
                Line::Open => print!("----    ----"),
                Line::Closed => print!("------------"),
            }
            println!()
        }
    }

    /// Returns the trigram obtained by reversing the order of the lines in this trigram.
    #[allow(dead_code)]
    pub fn reverse(&self) -> Trigram {
        let lines = [self.lines[2], self.lines[1], self.lines[0]];
        TRIGRAM_INDEX.get(&lines).copied().unwrap()
    }

    /// Returns the trigram obtained by flipping the lines in this trigram.
    #[allow(dead_code)]
    pub fn inverse(&self) -> Trigram {
        let lines = [
            self.lines[0].inverse(),
            self.lines[1].inverse(),
            self.lines[2].inverse(),
        ];
        TRIGRAM_INDEX.get(&lines).copied().unwrap()
    }

    /// Returns the trigram obtained by inverting the given line.
    #[allow(dead_code)]
    pub fn inverse_line(&self, line: TrigramLine) -> Trigram {
        let mut lines = self.lines;
        lines[line.line_to_index()] = lines[line.line_to_index()].inverse();
        TRIGRAM_INDEX.get(&lines).copied().unwrap()
    }
}

/// The list of all I Ching trigrams.
static TRIGRAMS: [(u8, [u8; 3]); 8] = [
    (1, [1, 1, 1]),
    (2, [1, 0, 0]),
    (3, [0, 1, 0]),
    (4, [0, 0, 1]),
    (5, [0, 0, 0]),
    (6, [0, 1, 1]),
    (7, [1, 0, 1]),
    (8, [1, 1, 0]),
];

/// Creates a trigram from a number and a list of lines.
fn create_trigram(number: u8, lines: [u8; 3]) -> Trigram {
    Trigram {
        number,
        lines: [lines[0].into(), lines[1].into(), lines[2].into()],
    }
}

/// Generates a map of lines to trigram number for fast lookup.
fn trigram_index() -> HashMap<[Line; 3], Trigram> {
    let mut index = HashMap::new();
    for (number, lines) in TRIGRAMS.iter() {
        let hex = create_trigram(*number, *lines);
        index.insert(hex.lines, hex);
    }
    index
}

lazy_static! {
    /// A map of lines to trigram number for fast lookup.
    static ref TRIGRAM_INDEX: HashMap<[Line; 3], Trigram> = trigram_index();
}

/// The possition of a line in a hexagram.
#[derive(Clone, Debug, PartialEq)]
pub enum HexagramLine {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
}

impl HexagramLine {
    /// Converts a line into an array index.
    fn line_to_index(&self) -> usize {
        match self {
            HexagramLine::First => 0,
            HexagramLine::Second => 1,
            HexagramLine::Third => 2,
            HexagramLine::Fourth => 3,
            HexagramLine::Fifth => 4,
            HexagramLine::Sixth => 5,
        }
    }
}

/// A single hexagram in a reading, consisting of six lines.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Hexagram {
    /// The number of the hexagram, from 1 to 64.
    pub number: u8,

    /// The lines of the hexagram. The first line is the bottom line, and the last line is the top
    /// one.
    pub lines: [Line; 6],
}

impl Hexagram {
    /// Prints the hexagram to the console.
    pub fn print(&self, changing_lines: Option<&HashSet<usize>>) {
        println!("     {}\n", self.number);
        for (i, line) in self.lines.iter().enumerate().rev() {
            match line {
                Line::Open => print!("----    ----"),
                Line::Closed => print!("------------"),
            }

            if changing_lines.map_or(false, |m| m.contains(&i)) {
                print!("  *")
            }
            println!()
        }
    }

    /// Returns the bottom and top trigrams of the hexagram.
    pub fn trigrams(&self) -> (Trigram, Trigram) {
        let lines = [self.lines[0], self.lines[1], self.lines[2]];
        let number = TRIGRAM_INDEX
            .get(&lines)
            .map(|trigram| trigram.number)
            .unwrap();
        let bottom = Trigram { number, lines };

        let lines = [self.lines[3], self.lines[4], self.lines[5]];
        let number = TRIGRAM_INDEX
            .get(&lines)
            .map(|trigram| trigram.number)
            .unwrap();
        let top = Trigram { number, lines };

        (bottom, top)
    }

    /// Counts the number of line changes between this and other hexagram.
    pub fn num_line_changes(&self, other: &Hexagram) -> usize {
        self.lines
            .iter()
            .zip(other.lines.iter())
            .filter(|(a, b)| a != b)
            .count()
    }

    /// Returns the hexagram obtained by inverting all lines in this hexagram.
    pub fn inverse(&self) -> Hexagram {
        let lines = [
            self.lines[0].inverse(),
            self.lines[1].inverse(),
            self.lines[2].inverse(),
            self.lines[3].inverse(),
            self.lines[4].inverse(),
            self.lines[5].inverse(),
        ];
        HEXAGRAM_INDEX.get(&lines).copied().unwrap()
    }

    /// Returns the hexagram obtained by inverting the bottom trigram of this hexagram.
    pub fn inverse_bottom_trigram(&self) -> Hexagram {
        let (bottom, top) = self.trigrams();
        let lines = [
            bottom.lines[0].inverse(),
            bottom.lines[1].inverse(),
            bottom.lines[2].inverse(),
            top.lines[0],
            top.lines[1],
            top.lines[2],
        ];
        HEXAGRAM_INDEX.get(&lines).copied().unwrap()
    }

    /// Returns the hexagram obtained by inverting the top trigram of this hexagram.
    pub fn inverse_top_trigram(&self) -> Hexagram {
        let (bottom, top) = self.trigrams();
        let lines = [
            bottom.lines[0],
            bottom.lines[1],
            bottom.lines[2],
            top.lines[0].inverse(),
            top.lines[1].inverse(),
            top.lines[2].inverse(),
        ];
        HEXAGRAM_INDEX.get(&lines).copied().unwrap()
    }

    /// Returns the hexagram obtained by inverting the given line. The line number is zero-based,
    /// with zero being the bottom line.
    pub fn inverse_line(&self, line: HexagramLine) -> Hexagram {
        let mut lines = self.lines;
        let index = line.line_to_index();
        lines[index] = lines[index].inverse();
        HEXAGRAM_INDEX.get(&lines).copied().unwrap()
    }

    /// Returns the hexagram obtained by reversing the order of the lines in this hexagram.
    pub fn reverse(&self) -> Hexagram {
        let lines = [
            self.lines[5],
            self.lines[4],
            self.lines[3],
            self.lines[2],
            self.lines[1],
            self.lines[0],
        ];
        HEXAGRAM_INDEX.get(&lines).copied().unwrap()
    }

    /// Returns the hexagram obtained by reversing the trigrams of this hexagram.
    #[allow(dead_code)]
    pub fn reverse_trigrams(&self) -> Hexagram {
        let (bottom, top) = self.trigrams();
        let lines = [
            top.lines[2],
            top.lines[1],
            top.lines[0],
            bottom.lines[2],
            bottom.lines[1],
            bottom.lines[0],
        ];
        HEXAGRAM_INDEX.get(&lines).copied().unwrap()
    }

    /// Returns the hexagram obtained by reversing the bottom trigram of this hexagram.
    pub fn reverse_bottom_trigram(&self) -> Hexagram {
        let (bottom, top) = self.trigrams();
        let lines = [
            bottom.lines[2],
            bottom.lines[1],
            bottom.lines[0],
            top.lines[0],
            top.lines[1],
            top.lines[2],
        ];
        HEXAGRAM_INDEX.get(&lines).copied().unwrap()
    }

    /// Returns the hexagram obtained by reversing the top trigram of this hexagram.
    pub fn reverse_top_trigram(&self) -> Hexagram {
        let (bottom, top) = self.trigrams();
        let lines = [
            bottom.lines[0],
            bottom.lines[1],
            bottom.lines[2],
            top.lines[2],
            top.lines[1],
            top.lines[0],
        ];
        HEXAGRAM_INDEX.get(&lines).copied().unwrap()
    }

    /// Returns the hexagram obtained by mirroring the trigrams of this hexagram along the dividing
    /// line between the two trigrams.
    pub fn mirror_trigrams(&self) -> Hexagram {
        let (bottom, top) = self.trigrams();
        let lines = [
            bottom.lines[2],
            bottom.lines[1],
            bottom.lines[0],
            top.lines[2],
            top.lines[1],
            top.lines[0],
        ];
        HEXAGRAM_INDEX.get(&lines).copied().unwrap()
    }
}

/// The list of all I Ching hexagrams.
pub static HEXAGRAMS: [(u8, [u8; 6]); 64] = [
    (1, [1, 1, 1, 1, 1, 1]),
    (2, [0, 0, 0, 0, 0, 0]),
    (3, [1, 0, 0, 0, 1, 0]),
    (4, [0, 1, 0, 0, 0, 1]),
    (5, [1, 1, 1, 0, 1, 0]),
    (6, [0, 1, 0, 1, 1, 1]),
    (7, [0, 1, 0, 0, 0, 0]),
    (8, [0, 0, 0, 0, 1, 0]),
    (9, [1, 1, 1, 0, 1, 1]),
    (10, [1, 1, 0, 1, 1, 1]),
    (11, [1, 1, 1, 0, 0, 0]),
    (12, [0, 0, 0, 1, 1, 1]),
    (13, [1, 0, 1, 1, 1, 1]),
    (14, [1, 1, 1, 1, 0, 1]),
    (15, [0, 0, 1, 0, 0, 0]),
    (16, [0, 0, 0, 1, 0, 0]),
    (17, [1, 0, 0, 1, 1, 0]),
    (18, [0, 1, 1, 0, 0, 1]),
    (19, [1, 1, 0, 0, 0, 0]),
    (20, [0, 0, 0, 0, 1, 1]),
    (21, [1, 0, 0, 1, 0, 1]),
    (22, [1, 0, 1, 0, 0, 1]),
    (23, [0, 0, 0, 0, 0, 1]),
    (24, [1, 0, 0, 0, 0, 0]),
    (25, [1, 0, 0, 1, 1, 1]),
    (26, [1, 1, 1, 0, 0, 1]),
    (27, [1, 0, 0, 0, 0, 1]),
    (28, [0, 1, 1, 1, 1, 0]),
    (29, [0, 1, 0, 0, 1, 0]),
    (30, [1, 0, 1, 1, 0, 1]),
    (31, [0, 0, 1, 1, 1, 0]),
    (32, [0, 1, 1, 1, 0, 0]),
    (33, [0, 0, 1, 1, 1, 1]),
    (34, [1, 1, 1, 1, 0, 0]),
    (35, [0, 0, 0, 1, 0, 1]),
    (36, [1, 0, 1, 0, 0, 0]),
    (37, [1, 0, 1, 0, 1, 1]),
    (38, [1, 1, 0, 1, 0, 1]),
    (39, [0, 0, 1, 0, 1, 0]),
    (40, [0, 1, 0, 1, 0, 0]),
    (41, [1, 1, 0, 0, 0, 1]),
    (42, [1, 0, 0, 0, 1, 1]),
    (43, [1, 1, 1, 1, 1, 0]),
    (44, [0, 1, 1, 1, 1, 1]),
    (45, [0, 0, 0, 1, 1, 0]),
    (46, [0, 1, 1, 0, 0, 0]),
    (47, [0, 1, 0, 1, 1, 0]),
    (48, [0, 1, 1, 0, 1, 0]),
    (49, [1, 0, 1, 1, 1, 0]),
    (50, [0, 1, 1, 1, 0, 1]),
    (51, [1, 0, 0, 1, 0, 0]),
    (52, [0, 0, 1, 0, 0, 1]),
    (53, [0, 0, 1, 0, 1, 1]),
    (54, [1, 1, 0, 1, 0, 0]),
    (55, [1, 0, 1, 1, 0, 0]),
    (56, [0, 0, 1, 1, 0, 1]),
    (57, [0, 1, 1, 0, 1, 1]),
    (58, [1, 1, 0, 1, 1, 0]),
    (59, [0, 1, 0, 0, 1, 1]),
    (60, [1, 1, 0, 0, 1, 0]),
    (61, [1, 1, 0, 0, 1, 1]),
    (62, [0, 0, 1, 1, 0, 0]),
    (63, [1, 0, 1, 0, 1, 0]),
    (64, [0, 1, 0, 1, 0, 1]),
];

/// Creates a hexagram from a number and a list of lines.
pub fn create_hexagram(number: u8, input_lines: [u8; 6]) -> Hexagram {
    let lines = input_lines.map(Line::from);
    Hexagram { number, lines }
}

/// Generate a map of lines to hexagrams for fast lookup.
fn hexagram_index() -> HashMap<[Line; 6], Hexagram> {
    let mut index = HashMap::new();
    for (number, lines) in HEXAGRAMS.iter() {
        let hex = create_hexagram(*number, *lines);
        index.insert(hex.lines, hex);
    }
    index
}

lazy_static! {
    /// A map of lines to hexagrams for fast lookup.
    pub static ref HEXAGRAM_INDEX: HashMap<[Line; 6], Hexagram> = hexagram_index();
}

/// A reading of the I Ching.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reading {
    /// The question asked of the I Ching.
    question: String,

    /// The present hexagram.
    present: Hexagram,

    /// The future hexagram, if any.
    future: Option<Hexagram>,

    /// The lines that are changing between the present and future hexagrams.
    changing_lines: HashSet<usize>,
}

impl Reading {
    /// Prints the reading to the console.
    pub fn print(&self) {
        if !self.question.is_empty() {
            println!("Question: {}", self.question);
        }
        println!("\nPresent Hexagram\n");
        self.present.print(Some(&self.changing_lines));

        match &self.future {
            Some(hex) => {
                println!("\nFuture Hexagram\n");
                hex.print(None);
            }
            None => (),
        }
    }
}

/// The method used to generate the reading.
#[derive(Clone, Debug, ValueEnum)]
pub enum ReadingMethod {
    /// A method using yarrow stalks. This is the traditional method, which is more involved. The
    /// probabilities that a yin or yang line will transform are not equal. This asymmetry reflects
    /// the traditional understanding of the intrinsic tendency of yin towards stability and of yang
    /// towards transformation.
    YarrowStalks,

    /// A method using random draws from a coin. This is a simplified method, which is easier to
    /// perform. The probabilities that a yin or yang line will transform are equal.
    Coin,
}

impl Display for ReadingMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadingMethod::YarrowStalks => write!(f, "yarrow-stalks"),
            ReadingMethod::Coin => write!(f, "coin"),
        }
    }
}

/// The method used to generate random numbers.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum RandomnessMode {
    /// Generate truly random numbers using random.org.
    Random,

    /// Generate pseudo-random numbers using the system's random number generator.
    Pseudorandom,
}

impl Display for RandomnessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RandomnessMode::Random => write!(f, "random"),
            RandomnessMode::Pseudorandom => write!(f, "pseudorandom"),
        }
    }
}

/// The URL to use for the coin method.
static COIN_READING_URL: &str =
    "https://www.random.org/integers/?num=1&min=2&max=3&col=1&base=10&format=plain&rnd=new";

/// Generates a random coin throw using random.org.
fn random_coin_throw() -> Result<u8> {
    let body = reqwest::blocking::get(COIN_READING_URL)?.text()?;
    let draw: u8 = body.trim().parse()?;
    Ok(draw)
}

/// Generates a pseudo-random coin throw using the system's random number generator.
fn pseudo_random_coin_throw() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..4)
}

/// Generates a coin throw based on the given randomness mode.
fn coin_draw(randomness: RandomnessMode) -> Result<u8> {
    match randomness {
        RandomnessMode::Random => random_coin_throw(),
        RandomnessMode::Pseudorandom => Ok(pseudo_random_coin_throw()),
    }
}

/// Generates a line using the given randomness mode.
fn coin_line(randomness: RandomnessMode) -> Result<u8> {
    // Throw the coin three times. One side of the coin is assigned a value of 2 and the other a
    // value of 3. The sum of the three throws is the value of the line, ranging from 6 to 9.
    let throw1 = coin_draw(randomness)?;
    let throw2 = coin_draw(randomness)?;
    let throw3 = coin_draw(randomness)?;
    Ok(throw1 + throw2 + throw3)
}

/// Generates a reading using the given randomness mode.
fn coin_reading(randomness: RandomnessMode) -> Result<Vec<u8>> {
    vec![0; 6].iter().map(|_| coin_line(randomness)).collect()
}

/// Generates a random number using random.org for use in the yarrow stalks method. The number
/// represents the number of stalks on the right pile after the split.
fn random_yarrow_stalks_split(num_stalks: u8) -> Result<u8> {
    // The max number to draw should be the number of stalks minus 2 so that the left pile always
    // has at least two stalks, since one will be removed from it.
    let url = format!(
        "https://www.random.org/integers/?num=1&min=1&max={}&col=1&base=10&format=plain&rnd=new",
        num_stalks - 2
    );
    let body = reqwest::blocking::get(url)?.text()?;
    let split: u8 = body.trim().parse()?;
    Ok(split)
}

/// Generates a random number using the system's random number generator for use in the yarrow
/// stalks method. The number represents the number of stalks on the right pile after the split.
fn pseudo_random_yarrow_stalks_split(num_stalks: u8) -> Result<u8> {
    // The max number to draw should be the number of stalks minus 2 so that the left pile always
    // has at least two stalks, since one will be removed from it.
    let mut rng = rand::thread_rng();
    Ok(rng.gen_range(1..num_stalks - 1))
}

/// Counts the reminder from a pile.
fn pile_reminder(pile_size: u8) -> u8 {
    let reminder = pile_size % 4;
    if reminder == 0 {
        4
    } else {
        reminder
    }
}

/// Splits the yarrow stalks into two piles, sets one stalk aside, and counts the remainder from the
/// two piles. This procedure is repeated three times to generate a line from the reading. Returns
/// the remaining stalks and the number of groups of four stalks that were counted.
fn yarrow_stalk_split(num_stalks: u8, randomness: RandomnessMode) -> Result<(u8, u8)> {
    // Split the stalks into two piles.
    let right = match randomness {
        RandomnessMode::Random => random_yarrow_stalks_split(num_stalks)?,
        RandomnessMode::Pseudorandom => pseudo_random_yarrow_stalks_split(num_stalks)?,
    };
    let left = num_stalks - right;

    // Take one stalk from the left pile and set it aside.
    let left = left - 1;

    // Count the groups of four and the remainder from the right pile.
    let right_reminder = pile_reminder(right);
    let right_groups = (right - right_reminder) / 4;

    // Count the groups of four and the remainder from the left pile.
    let left_reminder = pile_reminder(left);
    let left_groups = (left - left_reminder) / 4;

    // The remainders and the first stalk from the left pile are set aside.
    let new_num_stalks = num_stalks - right_reminder - left_reminder - 1;
    Ok((new_num_stalks, left_groups + right_groups))
}

/// Generates a line for a reading using the yarrow stalks method.
fn yarrow_stalk_line(randomness: RandomnessMode) -> Result<u8> {
    // Start with 49 stalks.
    let num_stalks = 49;

    // Split and count the remainders three times.
    let (num_stalks, _) = yarrow_stalk_split(num_stalks, randomness)?;
    let (num_stalks, _) = yarrow_stalk_split(num_stalks, randomness)?;
    let (_, groups) = yarrow_stalk_split(num_stalks, randomness)?;

    // The number of groups of four after the third split determines the line.
    Ok(groups)
}

/// Generates a reading using numbers from random.org and the yarrow stalks method.
fn yarrow_stalk_reading(randomness: RandomnessMode) -> Result<Vec<u8>> {
    vec![0; 6]
        .iter()
        .map(|_| yarrow_stalk_line(randomness))
        .collect()
}

/// Generate a reading of the I Ching using the given reading mode and randomness mode.
pub fn generate_reading(
    method: ReadingMethod,
    randomness: RandomnessMode,
    question: &str,
) -> Result<Reading> {
    // Generate the throws according to the reading method.
    let throws = match method {
        ReadingMethod::Coin => coin_reading(randomness)?,
        ReadingMethod::YarrowStalks => yarrow_stalk_reading(randomness)?,
    };

    // Convert the throws into the present and future lines.
    let mut present_lines = [Line::Open; 6];
    let mut future_lines = [Line::Open; 6];
    let mut changing_lines: HashSet<usize> = HashSet::new();
    for (i, throw) in throws.iter().enumerate() {
        match throw {
            6 => {
                present_lines[i] = Line::Open;
                future_lines[i] = Line::Closed;
                changing_lines.insert(i);
            }
            7 => {
                present_lines[i] = Line::Closed;
                future_lines[i] = Line::Closed;
            }
            8 => {
                present_lines[i] = Line::Open;
                future_lines[i] = Line::Open;
            }
            9 => {
                present_lines[i] = Line::Closed;
                future_lines[i] = Line::Open;
                changing_lines.insert(i);
            }
            _ => bail!("bad throw: {}", throw),
        }
    }

    // Build the present and future hexagrams.
    let present_hex = *HEXAGRAM_INDEX.get(&present_lines).ok_or(anyhow!(
        "cannot find hexagram for present lines: {:?}",
        present_lines
    ))?;
    let future_hex = *HEXAGRAM_INDEX.get(&future_lines).ok_or(anyhow!(
        "cannot find hexagram for future lines: {:?}",
        future_lines
    ))?;
    if present_lines == future_lines {
        Ok(Reading {
            question: question.to_string(),
            present: present_hex,
            future: None,
            changing_lines,
        })
    } else {
        Ok(Reading {
            question: question.to_string(),
            present: present_hex,
            future: Some(future_hex),
            changing_lines,
        })
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use super::*;

    /// A trait used to test an arbitrary method to generate an I Ching reading.
    trait ReadingGenerator {
        fn generate_reading(&self) -> Result<Vec<u8>>;
    }

    /// A reading method using the coin method with true randomness.
    struct CoinRandom {}
    impl ReadingGenerator for CoinRandom {
        fn generate_reading(&self) -> Result<Vec<u8>> {
            coin_reading(RandomnessMode::Random)
        }
    }

    /// A reading method using the coin method with pseudorandomness.
    struct CoinPseudorandom {}
    impl ReadingGenerator for CoinPseudorandom {
        fn generate_reading(&self) -> Result<Vec<u8>> {
            coin_reading(RandomnessMode::Pseudorandom)
        }
    }

    /// A reading method using the yarrow stalks method with true randomness.
    struct YarrowStalksRandom {}
    impl ReadingGenerator for YarrowStalksRandom {
        fn generate_reading(&self) -> Result<Vec<u8>> {
            yarrow_stalk_reading(RandomnessMode::Random)
        }
    }

    /// A reading method using the yarrow stalks method with pseudorandomness.
    struct YarrowStalksPseudorandom {}
    impl ReadingGenerator for YarrowStalksPseudorandom {
        fn generate_reading(&self) -> Result<Vec<u8>> {
            yarrow_stalk_reading(RandomnessMode::Pseudorandom)
        }
    }

    /// Verifies that the reading method generates a valid reading.
    struct ReadingVerifier {
        reading_method: Box<dyn ReadingGenerator>,
        num_readings: usize,
    }

    impl ReadingVerifier {
        /// Generates the given number of readings and verifies that all of them are valid.
        fn verify_reading(&self) -> Result<()> {
            for _ in 0..self.num_readings {
                let reading = self.reading_method.generate_reading()?;
                if reading.len() != 6 {
                    bail!("reading has wrong number of lines: {}", reading.len());
                }
                if reading.iter().any(|&x| x < 6 || x > 9) {
                    bail!("reading has invalid throw: {:?}", reading);
                }
            }
            Ok(())
        }
    }

    /// Verifies the coin method with true randomness.
    #[test]
    fn test_coin_random() -> Result<()> {
        ReadingVerifier {
            reading_method: Box::new(CoinRandom {}),
            num_readings: 1,
        }
        .verify_reading()
    }

    /// Verifies the coin method with pseudorandomness.
    #[test]
    fn test_coin_pseudorandom() -> Result<()> {
        ReadingVerifier {
            reading_method: Box::new(CoinPseudorandom {}),
            num_readings: 100,
        }
        .verify_reading()
    }

    /// Verifies the yarrow stalks method with true randomness.
    #[test]
    fn test_yarrow_stalks_random() -> Result<()> {
        ReadingVerifier {
            reading_method: Box::new(YarrowStalksRandom {}),
            num_readings: 1,
        }
        .verify_reading()
    }

    /// Verifies the yarrow stalks method with pseudorandomness.
    #[test]
    fn test_yarrow_stalks_pseudorandom() -> Result<()> {
        ReadingVerifier {
            reading_method: Box::new(YarrowStalksPseudorandom {}),
            num_readings: 100,
        }
        .verify_reading()
    }

    /// Verifies that the correct trigrams are extracted from an hexagram.
    #[test]
    fn hexagram_trigrams() -> Result<()> {
        for hexagram in HEXAGRAM_INDEX.values() {
            let (bottom, top) = hexagram.trigrams();

            assert_eq!(bottom.lines.len(), 3);
            assert_eq!(top.lines.len(), 3);
            assert_eq!(bottom.lines[0], hexagram.lines[0]);
            assert_eq!(bottom.lines[1], hexagram.lines[1]);
            assert_eq!(bottom.lines[2], hexagram.lines[2]);
            assert_eq!(top.lines[0], hexagram.lines[3]);
            assert_eq!(top.lines[1], hexagram.lines[4]);
            assert_eq!(top.lines[2], hexagram.lines[5]);
        }
        Ok(())
    }
}
