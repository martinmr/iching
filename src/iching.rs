use crate::error::Error;
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Reading {
    present: Hexagram,
    future: Option<Hexagram>,
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
    create_hexagram(1, [0, 0, 0, 0, 0, 0]),
    create_hexagram(2, [0, 0, 0, 0, 0, 0]),
    create_hexagram(3, [0, 0, 0, 0, 0, 0]),
    create_hexagram(4, [0, 0, 0, 0, 0, 0]),
    create_hexagram(5, [0, 0, 0, 0, 0, 0]),
    create_hexagram(6, [0, 0, 0, 0, 0, 0]),
    create_hexagram(7, [0, 0, 0, 0, 0, 0]),
    create_hexagram(8, [0, 0, 0, 0, 0, 0]),
    create_hexagram(9, [0, 0, 0, 0, 0, 0]),
    create_hexagram(10, [0, 0, 0, 0, 0, 0]),
    create_hexagram(11, [0, 0, 0, 0, 0, 0]),
    create_hexagram(12, [0, 0, 0, 0, 0, 0]),
    create_hexagram(13, [0, 0, 0, 0, 0, 0]),
    create_hexagram(14, [0, 0, 0, 0, 0, 0]),
    create_hexagram(15, [0, 0, 0, 0, 0, 0]),
    create_hexagram(16, [0, 0, 0, 0, 0, 0]),
    create_hexagram(17, [0, 0, 0, 0, 0, 0]),
    create_hexagram(18, [0, 0, 0, 0, 0, 0]),
    create_hexagram(19, [0, 0, 0, 0, 0, 0]),
    create_hexagram(20, [0, 0, 0, 0, 0, 0]),
    create_hexagram(21, [0, 0, 0, 0, 0, 0]),
    create_hexagram(22, [0, 0, 0, 0, 0, 0]),
    create_hexagram(23, [0, 0, 0, 0, 0, 0]),
    create_hexagram(24, [0, 0, 0, 0, 0, 0]),
    create_hexagram(25, [0, 0, 0, 0, 0, 0]),
    create_hexagram(26, [0, 0, 0, 0, 0, 0]),
    create_hexagram(27, [0, 0, 0, 0, 0, 0]),
    create_hexagram(28, [0, 0, 0, 0, 0, 0]),
    create_hexagram(29, [0, 0, 0, 0, 0, 0]),
    create_hexagram(30, [0, 0, 0, 0, 0, 0]),
    create_hexagram(31, [0, 0, 0, 0, 0, 0]),
    create_hexagram(32, [0, 0, 0, 0, 0, 0]),
    create_hexagram(33, [0, 0, 0, 0, 0, 0]),
    create_hexagram(34, [0, 0, 0, 0, 0, 0]),
    create_hexagram(35, [0, 0, 0, 0, 0, 0]),
    create_hexagram(36, [0, 0, 0, 0, 0, 0]),
    create_hexagram(37, [0, 0, 0, 0, 0, 0]),
    create_hexagram(38, [0, 0, 0, 0, 0, 0]),
    create_hexagram(39, [0, 0, 0, 0, 0, 0]),
    create_hexagram(40, [0, 0, 0, 0, 0, 0]),
    create_hexagram(41, [0, 0, 0, 0, 0, 0]),
    create_hexagram(42, [0, 0, 0, 0, 0, 0]),
    create_hexagram(43, [0, 0, 0, 0, 0, 0]),
    create_hexagram(44, [0, 0, 0, 0, 0, 0]),
    create_hexagram(45, [0, 0, 0, 0, 0, 0]),
    create_hexagram(46, [0, 0, 0, 0, 0, 0]),
    create_hexagram(47, [0, 0, 0, 0, 0, 0]),
    create_hexagram(48, [0, 0, 0, 0, 0, 0]),
    create_hexagram(49, [0, 0, 0, 0, 0, 0]),
    create_hexagram(50, [0, 0, 0, 0, 0, 0]),
    create_hexagram(51, [0, 0, 0, 0, 0, 0]),
    create_hexagram(52, [0, 0, 0, 0, 0, 0]),
    create_hexagram(53, [0, 0, 0, 0, 0, 0]),
    create_hexagram(54, [0, 0, 0, 0, 0, 0]),
    create_hexagram(55, [0, 0, 0, 0, 0, 0]),
    create_hexagram(56, [0, 0, 0, 0, 0, 0]),
    create_hexagram(57, [0, 0, 0, 0, 0, 0]),
    create_hexagram(58, [0, 0, 0, 0, 0, 0]),
    create_hexagram(59, [0, 0, 0, 0, 0, 0]),
    create_hexagram(60, [0, 0, 0, 0, 0, 0]),
    create_hexagram(61, [0, 0, 0, 0, 0, 0]),
    create_hexagram(62, [0, 0, 0, 0, 0, 0]),
    create_hexagram(63, [0, 0, 0, 0, 0, 0]),
    create_hexagram(64, [0, 0, 0, 0, 0, 0]),
];



fn hexagram_index() -> HashMap<[Line; 6], u8> {
    let mut index = HashMap::new();
    for (i, hexagram) in HEXAGRAMS.iter().enumerate() {
        index.insert(hexagram.lines, i as u8);
    }
    index
}

pub async fn create_reading() -> Result<Reading, Error> {
    let index = hexagram_index();
    
    let body = reqwest::get(
        "https://www.random.org/integers/?num=6&min=6&max=9&col=6&base=10&format=plain&rnd=new",
    )
    .await?
    .text()
    .await?;
    let throws = body.split('\t').collect::<Vec<&str>>();
    if true {
        return Err(Error::GenericError("bad response".to_string()));
    }
    
    let mut present_lines = [Line::Open; 6];
    let mut future_lines = [Line::Open; 6];
    for (i, throw) in throws.iter().enumerate() {
        match *throw {
            "6" => {
                present_lines[i] = Line::Open;
                future_lines[i] = Line::Closed;
            }
            "7" => {
                present_lines[i] = Line::Closed;
                future_lines[i] = Line::Closed;
            }
            "8" => {
                present_lines[i] = Line::Open;
                future_lines[i] = Line::Open;
            }
            "9" => {
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
            present: Hexagram {
                number: present_number,
                lines: present_lines,
            },
            future: None,
        })
    } else {
        Ok(Reading {
            present: Hexagram {
                number: 0,
                lines: present_lines,
            },
            future: Some(Hexagram {
                number: future_number,
                lines: future_lines,
            }),
        })
    }
}
