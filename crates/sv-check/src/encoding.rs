//! Base64, written and read in one place (backlog 0455), rather than as the five copies it was, two of whose decoders
//! disagreed on which alphabet a web token may use.
//!
//! Reading is strict: each text decodes one way only, so the bits a short last group does not use must be zero, and a
//! character outside the alphabet asked for is refused. A JSON Web Token's parts are base64url, the alphabet made for
//! web addresses, with no padding (RFC 7515, section 2): a part written with `+` or `/` is not a token's part, for
//! every check that reads one.

/// Which 64 characters stand for the six-bit values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alphabet {
    /// RFC 4648's standard alphabet, ending `+/`: SSH keys, PEM files, and the request handed to the sidecar.
    Standard,
    /// RFC 4648's alphabet for web addresses, ending `-_`: a JSON Web Token's parts.
    Web,
}

/// Whether a text read back must end in `=` to a whole number of groups of four.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Padding {
    /// It must, as an SSH key or a PEM file is written.
    Required,
    /// It may, or may leave it off, as a web token's part is written (and some libraries pad anyway).
    Optional,
}

impl Alphabet {
    fn characters(self) -> &'static [u8; 64] {
        match self {
            Alphabet::Standard => {
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
            }
            Alphabet::Web => b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
        }
    }
}

/// `bytes` in base64, padded with `=` or not.
pub fn base64(bytes: &[u8], alphabet: Alphabet, pad: bool) -> String {
    let characters = alphabet.characters();
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |n, (i, b)| n | u32::from(*b) << (16 - 8 * i));
        for i in 0..=chunk.len() {
            out.push(characters[(n >> (18 - 6 * i) & 63) as usize] as char);
        }
        if pad {
            for _ in chunk.len()..3 {
                out.push('=');
            }
        }
    }
    out
}

/// The bytes `text` stands for, or `None` when it is not base64 in `alphabet` with `padding`.
pub fn unbase64(text: &str, alphabet: Alphabet, padding: Padding) -> Option<Vec<u8>> {
    let mut text = text.as_bytes().to_vec();
    if padding == Padding::Optional && !text.ends_with(b"=") {
        // An unpadded text is read as the padded one it stands for; one character over a group never is one.
        match text.len() % 4 {
            0 => {}
            1 => return None,
            short => text.extend(std::iter::repeat_n(b'=', 4 - short)),
        }
    }
    if !text.len().is_multiple_of(4) {
        return None;
    }
    let characters = alphabet.characters();
    let mut out = Vec::with_capacity(text.len() / 4 * 3);
    for (g, group) in text.chunks(4).enumerate() {
        let last = g == text.len() / 4 - 1;
        let pads = group.iter().rev().take_while(|b| **b == b'=').count();
        if pads > 2 || (pads > 0 && !last) {
            return None;
        }
        let mut n = 0u32;
        for (i, b) in group[..4 - pads].iter().enumerate() {
            let v = characters.iter().position(|a| a == b)? as u32;
            n |= v << (18 - 6 * i);
        }
        let bytes = [(n >> 16) as u8, (n >> 8) as u8, n as u8];
        let keep = 3 - pads;
        // Bits a padded group does not use must be zero, so each text decodes one way only.
        if bytes[keep..].iter().any(|b| *b != 0) {
            return None;
        }
        out.extend_from_slice(&bytes[..keep]);
    }
    Some(out)
}

/// A JSON Web Token's part as a token writes it: base64url without padding (RFC 7515, section 2).
pub fn token_part(bytes: &[u8]) -> String {
    base64(bytes, Alphabet::Web, false)
}

/// A JSON Web Token's part read back: base64url, padded or not; `None` for anything else, `+` and `/` among it.
pub fn read_token_part(text: &str) -> Option<Vec<u8>> {
    unbase64(text, Alphabet::Web, Padding::Optional)
}

#[cfg(test)]
#[path = "encoding_tests.rs"]
mod tests;
