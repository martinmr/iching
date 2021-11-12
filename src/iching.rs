#[derive(Copy, Clone)]
pub enum Line {
    Open,
    Closed,
}

pub struct Hexagram {
    pub number: u8,
    pub lines: [Line; 6],
}

pub enum LineReading {
    Open,
    OpenClosing,
    Closed,
    ClosedOpening
}

pub static HEXAGRAMS: [Hexagram; 64] = [
    create_hexagram(0, [0, 0, 0, 0, 0, 0]),
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
];

impl Line {
    pub const fn from_u8(n: u8) -> Line {
        match n {
            0 => Line::Open,
            _ => Line::Closed,
        }
    }
}

pub const fn create_hexagram(hexagram_num: u8, input_lines: [u8; 6]) -> Hexagram {
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

pub fn create_reading() -> (Hexagram, Option<Hexagram>) {
    
}
