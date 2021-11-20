use crate::error::Error;
use rand::Rng;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Line {
    Open,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Hexagram {
    pub number: u8,
    pub lines: [Line; 6],
}

impl Hexagram {
    pub fn print(&self, changing_lines: Option<&HashSet<usize>>) {
        println!("     {}\n", self.number);
        for (i, line) in self.lines.iter().enumerate() {
            match line {
                Line::Open => print!("----    ----"),
                Line::Closed => print!("------------"),
            }

            if changing_lines != None && changing_lines.unwrap().contains(&i) {
                print!("  *")
            }
            println!("")
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reading {
    question: String,
    present: Hexagram,
    future: Option<Hexagram>,
    changing_lines: HashSet<usize>,
}

impl Reading {
    pub fn print(&self) {
        if self.question.len() > 0 {
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    Random,
    Pseudorandom,
}

impl From<&str> for Mode {
    fn from(val: &str) -> Self {
        match val {
            "random" => Mode::Random,
            "pseudorandom" => Mode::Pseudorandom,
            _ => Mode::Random,
        }
    }
}

fn create_hexagram(number: u8, input_lines: [u8; 6]) -> Hexagram {
    let lines = input_lines.map(|l| Line::from(l));
    Hexagram {
        number: number,
        lines: lines,
    }
}

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

fn hexagram_index() -> HashMap<[Line; 6], Hexagram> {
    let mut index = HashMap::new();
    for (number, lines) in HEXAGRAMS.iter() {
        let hex = create_hexagram(*number, *lines);
        index.insert(hex.lines, hex);
    }
    index
}

static READING_URL: &str =
    "https://www.random.org/integers/?num=1&min=6&max=9&col=6&base=10&format=plain&rnd=new";

pub fn random_draw() -> Result<u8, Error> {
    let body = reqwest::blocking::get(READING_URL)?.text()?;
    let draw: u8 = body.trim().parse()?;
    Ok(draw)
}

pub fn random_reading() -> Result<Vec<u8>, Error> {
    vec![0; 6].iter().map(|_| random_draw()).collect()
}

pub fn pseudorandom_reading() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut throws = Vec::new();
    for _ in 0..6 {
        throws.push(rng.gen_range(6..10))
    }
    throws
}

pub fn generate_reading(mode: Mode, question: &str) -> Result<Reading, Error> {
    let index = hexagram_index();

    let throws = if mode == Mode::Random {
        random_reading()?
    } else {
        pseudorandom_reading()
    };

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
            _ => return Err(Error::InvalidReading),
        }
    }

    let present_hex = *index.get(&present_lines).ok_or(Error::InvalidReading)?;
    let future_hex = *index.get(&future_lines).ok_or(Error::InvalidReading)?;

    if present_lines == future_lines {
        Ok(Reading {
            question: question.to_string(),
            present: present_hex,
            future: None,
            changing_lines: changing_lines,
        })
    } else {
        Ok(Reading {
            question: question.to_string(),
            present: present_hex,
            future: Some(future_hex),
            changing_lines: changing_lines,
        })
    }
}
