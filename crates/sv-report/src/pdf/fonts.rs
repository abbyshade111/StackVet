//! The five fonts every PDF reader has without being given them (Helvetica, its bold, oblique, and bold oblique, and
//! Courier), so `report.pdf` carries no font file and `sv` downloads none. The price is the alphabet: these fonts show
//! the Windows-1252 characters (Western European letters and common punctuation) and nothing else. [`encode`] says what
//! happens to the rest: a few arrows and signs become their ASCII spelling, invisible characters are left out, and any
//! other character prints as `?`. [`representable`] says which those are, for the test that keeps `sv`'s own words
//! inside the fonts.
//!
//! The widths are the Adobe core-14 font metrics, in thousandths of the font size, as v1's writer had them (v1's
//! ADR-013). Every reader draws these fonts at these widths, or with a substitute made to the same widths (Arial,
//! Liberation Sans, Nimbus Sans), so a line measured here fits where it is drawn.

/// One of the fonts, by what it is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Font {
    Regular,
    Bold,
    Italic,
    BoldItalic,
    Mono,
}

impl Font {
    /// Every font, in the order the PDF names them `/F1` to `/F5`.
    pub const ALL: [Font; 5] = [
        Font::Regular,
        Font::Bold,
        Font::Italic,
        Font::BoldItalic,
        Font::Mono,
    ];

    /// The font's name in the PDF's resources.
    pub fn key(self) -> &'static str {
        match self {
            Font::Regular => "F1",
            Font::Bold => "F2",
            Font::Italic => "F3",
            Font::BoldItalic => "F4",
            Font::Mono => "F5",
        }
    }

    /// The name every PDF reader knows it by.
    pub fn base_name(self) -> &'static str {
        match self {
            Font::Regular => "Helvetica",
            Font::Bold => "Helvetica-Bold",
            Font::Italic => "Helvetica-Oblique",
            Font::BoldItalic => "Helvetica-BoldOblique",
            Font::Mono => "Courier",
        }
    }

    /// The font a key names, for reading a PDF back.
    pub fn from_key(key: &str) -> Option<Font> {
        Font::ALL.into_iter().find(|f| f.key() == key)
    }

    /// This font made bold, or kept bold.
    pub fn bold(self) -> Font {
        match self {
            Font::Regular | Font::Bold => Font::Bold,
            Font::Italic | Font::BoldItalic => Font::BoldItalic,
            Font::Mono => Font::Mono,
        }
    }

    /// This font made oblique, or kept oblique.
    pub fn italic(self) -> Font {
        match self {
            Font::Regular | Font::Italic => Font::Italic,
            Font::Bold | Font::BoldItalic => Font::BoldItalic,
            Font::Mono => Font::Mono,
        }
    }

    fn is_bold(self) -> bool {
        matches!(self, Font::Bold | Font::BoldItalic)
    }
}

/// Widths of the characters 32 to 126 in Helvetica (and Helvetica-Oblique, the same shapes slanted).
const HELVETICA: [u16; 95] = [
    278, 278, 355, 556, 556, 889, 667, 191, 333, 333, 389, 584, 278, 333, 278, 278, 556, 556, 556,
    556, 556, 556, 556, 556, 556, 556, 278, 278, 584, 584, 584, 556, 1015, 667, 667, 722, 722, 667,
    611, 778, 722, 278, 500, 667, 556, 833, 722, 778, 667, 778, 722, 667, 611, 722, 667, 944, 667,
    667, 611, 278, 278, 278, 469, 556, 333, 556, 556, 500, 556, 556, 278, 556, 556, 222, 222, 500,
    222, 833, 556, 556, 556, 556, 333, 500, 278, 556, 500, 722, 500, 500, 500, 334, 260, 334, 584,
];

/// The same for Helvetica-Bold (and Helvetica-BoldOblique).
const HELVETICA_BOLD: [u16; 95] = [
    278, 333, 474, 556, 556, 889, 722, 238, 333, 333, 389, 584, 278, 333, 278, 278, 556, 556, 556,
    556, 556, 556, 556, 556, 556, 556, 333, 333, 584, 584, 584, 611, 975, 722, 722, 722, 722, 667,
    611, 778, 722, 278, 556, 722, 611, 833, 722, 778, 667, 778, 722, 667, 611, 722, 667, 944, 667,
    667, 611, 333, 278, 333, 584, 556, 333, 556, 611, 556, 611, 556, 333, 611, 611, 278, 278, 556,
    278, 889, 611, 611, 611, 611, 389, 556, 333, 611, 556, 778, 556, 556, 500, 389, 280, 389, 584,
];

/// The Windows-1252 bytes 0x80 to 0x9F, which are not the Unicode characters of the same numbers.
const CP1252: [(char, u8); 27] = [
    ('\u{20ac}', 0x80),
    ('\u{201a}', 0x82),
    ('\u{0192}', 0x83),
    ('\u{201e}', 0x84),
    ('\u{2026}', 0x85),
    ('\u{2020}', 0x86),
    ('\u{2021}', 0x87),
    ('\u{02c6}', 0x88),
    ('\u{2030}', 0x89),
    ('\u{0160}', 0x8a),
    ('\u{2039}', 0x8b),
    ('\u{0152}', 0x8c),
    ('\u{017d}', 0x8e),
    ('\u{2018}', 0x91),
    ('\u{2019}', 0x92),
    ('\u{201c}', 0x93),
    ('\u{201d}', 0x94),
    ('\u{2022}', 0x95),
    ('\u{2013}', 0x96),
    ('\u{2014}', 0x97),
    ('\u{02dc}', 0x98),
    ('\u{2122}', 0x99),
    ('\u{0161}', 0x9a),
    ('\u{203a}', 0x9b),
    ('\u{0153}', 0x9c),
    ('\u{017e}', 0x9e),
    ('\u{0178}', 0x9f),
];

/// Characters the fonts lack that have a plain spelling in the ones they have.
const SPELLED: [(char, &str); 12] = [
    ('\u{2192}', "->"),
    ('\u{2190}', "<-"),
    ('\u{2194}', "<->"),
    ('\u{21d2}', "=>"),
    ('\u{2265}', ">="),
    ('\u{2264}', "<="),
    ('\u{2260}', "!="),
    ('\u{2010}', "-"),
    ('\u{2011}', "-"),
    ('\u{2012}', "-"),
    ('\u{2212}', "-"),
    ('\u{2032}', "'"),
];

/// Characters with nothing to draw: joiners, marks that choose a form, and the byte-order mark.
fn invisible(c: char) -> bool {
    matches!(c, '\u{200b}'..='\u{200f}' | '\u{2060}' | '\u{feff}' | '\u{fe00}'..='\u{fe0f}')
}

/// The byte for one character in the fonts' encoding (WinAnsiEncoding), if it has one.
fn byte(c: char) -> Option<u8> {
    match c as u32 {
        32..=126 | 0xa0..=0xff => Some(c as u32 as u8),
        _ => CP1252.iter().find(|(u, _)| *u == c).map(|(_, b)| *b),
    }
}

/// `text` as the bytes the fonts draw. Tabs and line ends become spaces; see the module comment for the rest.
pub fn encode(text: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(text.len());
    for c in text.chars() {
        if matches!(c, '\t' | '\n' | '\r') {
            out.push(b' ');
        } else if let Some(b) = byte(c) {
            out.push(b);
        } else if let Some((_, s)) = SPELLED.iter().find(|(u, _)| *u == c) {
            out.extend(s.bytes());
        } else if invisible(c) || c.is_control() {
            continue;
        } else {
            out.push(b'?');
        }
    }
    out
}

/// Whether `c` reaches the page as itself or as its plain spelling, rather than as `?`.
pub fn representable(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\r')
        || byte(c).is_some()
        || SPELLED.iter().any(|(u, _)| *u == c)
        || invisible(c)
}

/// The character a byte of the fonts' encoding stands for, for reading a PDF back.
pub fn decode(b: u8) -> char {
    match b {
        0x80..=0x9f => CP1252
            .iter()
            .find(|(_, x)| *x == b)
            .map_or('?', |(u, _)| *u),
        _ => b as char,
    }
}

/// The letter an accented Latin-1 letter is drawn as wide as.
fn base_letter(b: u8) -> Option<u8> {
    Some(match b {
        0xc0..=0xc5 => b'A',
        0xc7 => b'C',
        0xc8..=0xcb => b'E',
        0xcc..=0xcf => b'I',
        0xd1 => b'N',
        0xd2..=0xd6 | 0xd8 => b'O',
        0xd9..=0xdc => b'U',
        0xdd => b'Y',
        0xe0..=0xe5 => b'a',
        0xe7 => b'c',
        0xe8..=0xeb => b'e',
        0xec..=0xef => b'i',
        0xf1 => b'n',
        0xf2..=0xf6 | 0xf8 => b'o',
        0xf9..=0xfc => b'u',
        0xfd | 0xff => b'y',
        0x8a => b'S',
        0x9a => b's',
        0x8e => b'Z',
        0x9e => b'z',
        0x9f => b'Y',
        _ => return None,
    })
}

/// The width of one byte in `font`, in thousandths of the font size.
fn byte_width(b: u8, font: Font) -> u16 {
    if font == Font::Mono {
        return 600;
    }
    let table = if font.is_bold() {
        &HELVETICA_BOLD
    } else {
        &HELVETICA
    };
    if (32..=126).contains(&b) {
        return table[usize::from(b - 32)];
    }
    if let Some(letter) = base_letter(b) {
        return table[usize::from(letter - 32)];
    }
    let bold = font.is_bold();
    match b {
        0x85 | 0x97 | 0x99 | 0x89 | 0x8c | 0xc6 => 1000,
        0x91 | 0x92 | 0x82 => {
            if bold {
                278
            } else {
                222
            }
        }
        0x93 | 0x94 | 0x84 => {
            if bold {
                500
            } else {
                333
            }
        }
        0x95 => 350,
        0x96 | 0x80 | 0x86 | 0x87 => 556,
        0xa0 => 278,
        0xb7 => {
            if bold {
                333
            } else {
                278
            }
        }
        0xd7 | 0xb1 | 0xac | 0xf7 => 584,
        0xb0 => 400,
        0xa9 | 0xae => 737,
        0xe6 | 0x9c => {
            if bold {
                944
            } else {
                889
            }
        }
        0xdf | 0xd8 => 611,
        // Anything else: as wide as the widest ordinary letter, so a line measured here is never too short.
        _ => 1000,
    }
}

/// How wide `text` is drawn in `font` at `size` points.
pub fn width(text: &str, font: Font, size: f32) -> f32 {
    width_of_bytes(&encode(text), font, size)
}

/// The same, for bytes already encoded.
pub fn width_of_bytes(bytes: &[u8], font: Font, size: f32) -> f32 {
    let total: u32 = bytes.iter().map(|b| u32::from(byte_width(*b, font))).sum();
    total as f32 * size / 1000.0
}
