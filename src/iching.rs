use anyhow::{anyhow, bail, Result};
use clap::ValueEnum;
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

impl From<u8> for Line {
    fn from(n: u8) -> Self {
        match n {
            0 => Line::Open,
            _ => Line::Closed,
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
}

/// The list of all I Ching hexagrams.
static HEXAGRAMS: [(u8, [u8; 6]); 64] = [
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
fn create_hexagram(number: u8, input_lines: [u8; 6]) -> Hexagram {
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
    /// Prints the reading to stdout.
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
pub(crate) enum ReadingMethod {
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
pub(crate) enum RandomnessMode {
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
    "https://www.random.org/integers/?num=1&min=6&max=9&col=1&base=10&format=plain&rnd=new";

/// Generates a random number using random.org for use in the coin method.
fn random_coin_draw() -> Result<u8> {
    let body = reqwest::blocking::get(COIN_READING_URL)?.text()?;
    let draw: u8 = body.trim().parse()?;
    Ok(draw)
}

/// Generates a random number using the system's random number generator for use in the coin method.
fn pseudo_random_coin_draw() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(6..10)
}

/// Generates a reading using numbers from random.org and the coin method.
fn coin_reading(randomness: RandomnessMode) -> Result<Vec<u8>> {
    match randomness {
        RandomnessMode::Random => vec![0; 6].iter().map(|_| random_coin_draw()).collect(),
        RandomnessMode::Pseudorandom => Ok(vec![0; 6]
            .iter()
            .map(|_| pseudo_random_coin_draw())
            .collect()),
    }
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
pub(crate) fn generate_reading(
    method: ReadingMethod,
    randomness: RandomnessMode,
    question: &str,
) -> Result<Reading> {
    // Create the hexagram index.
    let index = hexagram_index();

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
    let present_hex = *index.get(&present_lines).ok_or(anyhow!(
        "cannot find hexagram for present lines: {:?}",
        present_lines
    ))?;
    let future_hex = *index.get(&future_lines).ok_or(anyhow!(
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
mod test {}
