use crate::error::Error;
use rand::Rng;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Line {
    Open,
    Closed,
}

impl Line {
    const fn from_u8(n: u8) -> Line {
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
    pub fn print(&self) {
        println!("     {}\n", self.number);
        for line in self.lines.iter() {
            match line {
                Line::Open => println!("----    ----"),
                Line::Closed => println!("------------"),
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reading {
    question: String,
    present: Hexagram,
    future: Option<Hexagram>,
}

impl Reading {
    pub fn print(&self) {
        if self.question.len() > 0 {
            println!("Question: {}", self.question);
        }
        println!("\nPresent Hexagram\n");
        self.present.print();

        match self.future {
            Some(hex) => {
                println!("\nFuture Hexagram\n");
                hex.print();
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

const fn create_hexagram(hexagram_num: u8, input_lines: [u8; 6]) -> Hexagram {
    let lines = [
        Line::from_u8(input_lines[0]),
        Line::from_u8(input_lines[1]),
        Line::from_u8(input_lines[2]),
        Line::from_u8(input_lines[3]),
        Line::from_u8(input_lines[4]),
        Line::from_u8(input_lines[5]),
    ];

    Hexagram {
        lines: lines,
        number: hexagram_num,
    }
}

static HEXAGRAMS: [Hexagram; 64] = [
    create_hexagram(1, [1, 1, 1, 1, 1, 1]),
    create_hexagram(2, [0, 0, 0, 0, 0, 0]),
    create_hexagram(3, [1, 0, 0, 0, 1, 0]),
    create_hexagram(4, [0, 1, 0, 0, 0, 1]),
    create_hexagram(5, [1, 1, 1, 0, 1, 0]),
    create_hexagram(6, [0, 1, 0, 1, 1, 1]),
    create_hexagram(7, [0, 1, 0, 0, 0, 0]),
    create_hexagram(8, [0, 0, 0, 0, 1, 0]),
    create_hexagram(9, [1, 1, 1, 0, 1, 1]),
    create_hexagram(10, [1, 1, 0, 1, 1, 1]),
    create_hexagram(11, [1, 1, 1, 0, 0, 0]),
    create_hexagram(12, [0, 0, 0, 1, 1, 1]),
    create_hexagram(13, [1, 0, 1, 1, 1, 1]),
    create_hexagram(14, [1, 1, 1, 1, 0, 1]),
    create_hexagram(15, [0, 0, 1, 0, 0, 0]),
    create_hexagram(16, [0, 0, 0, 1, 0, 0]),
    create_hexagram(17, [1, 0, 0, 1, 1, 0]),
    create_hexagram(18, [0, 1, 1, 0, 0, 1]),
    create_hexagram(19, [1, 1, 0, 0, 0, 0]),
    create_hexagram(20, [0, 0, 0, 0, 1, 1]),
    create_hexagram(21, [1, 0, 0, 1, 0, 1]),
    create_hexagram(22, [1, 0, 1, 0, 0, 1]),
    create_hexagram(23, [0, 0, 0, 0, 0, 1]),
    create_hexagram(24, [1, 0, 0, 0, 0, 0]),
    create_hexagram(25, [1, 0, 0, 1, 1, 1]),
    create_hexagram(26, [1, 1, 1, 0, 0, 1]),
    create_hexagram(27, [1, 0, 0, 0, 0, 1]),
    create_hexagram(28, [0, 1, 1, 1, 1, 0]),
    create_hexagram(29, [0, 1, 0, 0, 1, 0]),
    create_hexagram(30, [1, 0, 1, 1, 0, 1]),
    create_hexagram(31, [0, 0, 1, 1, 1, 0]),
    create_hexagram(32, [0, 1, 1, 1, 0, 0]),
    create_hexagram(33, [0, 0, 1, 1, 1, 1]),
    create_hexagram(34, [1, 1, 1, 1, 0, 0]),
    create_hexagram(35, [0, 0, 0, 1, 0, 1]),
    create_hexagram(36, [1, 0, 1, 0, 0, 0]),
    create_hexagram(37, [1, 0, 1, 0, 1, 1]),
    create_hexagram(38, [1, 1, 0, 1, 0, 1]),
    create_hexagram(39, [0, 0, 1, 0, 1, 0]),
    create_hexagram(40, [0, 1, 0, 1, 0, 0]),
    create_hexagram(41, [1, 1, 0, 0, 0, 1]),
    create_hexagram(42, [1, 0, 0, 0, 1, 1]),
    create_hexagram(43, [1, 1, 1, 1, 1, 0]),
    create_hexagram(44, [0, 1, 1, 1, 1, 1]),
    create_hexagram(45, [0, 0, 0, 1, 1, 0]),
    create_hexagram(46, [0, 1, 1, 0, 0, 0]),
    create_hexagram(47, [0, 1, 0, 1, 1, 0]),
    create_hexagram(48, [0, 1, 1, 0, 1, 0]),
    create_hexagram(49, [1, 0, 1, 1, 1, 0]),
    create_hexagram(50, [0, 1, 1, 1, 0, 1]),
    create_hexagram(51, [1, 0, 0, 1, 0, 0]),
    create_hexagram(52, [0, 0, 1, 0, 0, 1]),
    create_hexagram(53, [0, 0, 1, 0, 1, 1]),
    create_hexagram(54, [1, 1, 0, 1, 0, 0]),
    create_hexagram(55, [1, 0, 1, 1, 0, 0]),
    create_hexagram(56, [0, 0, 1, 1, 0, 1]),
    create_hexagram(57, [0, 1, 1, 0, 1, 1]),
    create_hexagram(58, [1, 1, 0, 1, 1, 0]),
    create_hexagram(59, [0, 1, 0, 0, 1, 1]),
    create_hexagram(60, [1, 1, 0, 0, 1, 0]),
    create_hexagram(61, [1, 1, 0, 0, 1, 1]),
    create_hexagram(62, [0, 0, 1, 1, 0, 0]),
    create_hexagram(63, [1, 0, 1, 0, 1, 0]),
    create_hexagram(64, [0, 1, 0, 1, 0, 1]),
];

fn hexagram_index() -> HashMap<[Line; 6], u8> {
    let mut index = HashMap::new();
    for (i, hexagram) in HEXAGRAMS.iter().enumerate() {
        index.insert(hexagram.lines, i as u8);
    }
    index
}

pub fn random_reading() -> Result<Vec<u8>, Error> {
    let body = reqwest::blocking::get(
        "https://www.random.org/integers/?num=6&min=6&max=9&col=6&base=10&format=plain&rnd=new",
    )?
    .text()?;

    let throws = body
        .trim()
        .split('\t')
        .map(|s| -> u8 { s.parse::<u8>().unwrap() })
        .collect::<Vec<u8>>();
    if throws.len() != 6 {
        return Err(Error::GenericError("bad response".to_string()));
    }
    Ok(throws)
}

pub fn pseudorandom_reading() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut throws = Vec::new();
    for _ in 0..6 {
        throws.push(rng.gen())
    }
    throws
}

pub fn create_reading(mode: Mode, question: &str) -> Result<Reading, Error> {
    let index = hexagram_index();

    let throws = if mode == Mode::Random {
        random_reading()?
    } else {
        pseudorandom_reading()
    };

    let mut present_lines = [Line::Open; 6];
    let mut future_lines = [Line::Open; 6];
    for (i, throw) in throws.iter().enumerate() {
        match throw {
            6 => {
                present_lines[i] = Line::Open;
                future_lines[i] = Line::Closed;
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
            }
            _ => {
                return Err(Error::GenericError("".to_string()));
            }
        }
    }
    let present_number = *index.get(&present_lines).unwrap();
    let future_number = *index.get(&future_lines).unwrap();

    if present_lines == future_lines {
        Ok(Reading {
            question: question.to_string(),
            present: Hexagram {
                number: present_number,
                lines: present_lines,
            },
            future: None,
        })
    } else {
        Ok(Reading {
            question: question.to_string(),
            present: Hexagram {
                number: present_number,
                lines: present_lines,
            },
            future: Some(Hexagram {
                number: future_number,
                lines: future_lines,
            }),
        })
    }
}
