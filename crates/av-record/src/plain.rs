//! The plain helpers the records need and `sv` already had: a SHA-256, a time written out, a refusal of a link that
//! says what to do instead, making a folder below another one level at a time, and hiding what is typed for a
//! passphrase (`quiet`, moved on 21 November 2026 for `av checkpoint`). Moved here from `sv-cli` word for
//! word on 21 November 2026 (backlog 0705, ADR-159), and `sv-cli` reads each under its old name, so there is one of
//! each.

use anyhow::{Context, Result};
use std::path::{Component, Path, PathBuf};

/// How the line of a report's seal file that holds the seal begins. The build-loop record looks for it to tell a
/// report folder `sv` sealed; `sv-cli`'s `report_seal` writes it.
pub const REPORT_SEAL_LINE: &str = "seal: ";

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// The SHA-256 of `data`, as lowercase hex.
pub fn sha256(data: &[u8]) -> String {
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let mut padded = data.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&((data.len() as u64) * 8).to_be_bytes());
    for start in (0..padded.len()).step_by(64) {
        let block = &padded[start..start + 64];
        let mut w = [0u32; 64];
        for (i, slot) in w.iter_mut().take(16).enumerate() {
            let at = i * 4;
            *slot = u32::from_be_bytes([block[at], block[at + 1], block[at + 2], block[at + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (slot, value) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *slot = slot.wrapping_add(value);
        }
    }
    h.iter().map(|word| format!("{word:08x}")).collect()
}

/// `seconds` since 1 January 1970 as `2026-09-27T14:03:09Z`.
pub fn utc_time(seconds: u64) -> String {
    let days = (seconds / 86_400) as i64;
    let rest = seconds % 86_400;
    // Days since 1970-01-01 to a calendar date (Howard Hinnant's civil_from_days).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { year + 1 } else { year };
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rest / 3600,
        rest % 3600 / 60,
        rest % 60
    )
}

/// Refuses a path that is a link, whatever it points to, saying so in the owner's terms and saying
/// what to do instead.
pub fn refuse_link(path: &Path, what_to_do: &str) -> Result<()> {
    if let Ok(meta) = std::fs::symlink_metadata(path)
        && meta.file_type().is_symlink()
    {
        return Err(Remedy::error(
            format!(
                "{} is a link to somewhere else, so sv does not read or write through it.",
                path.display()
            ),
            what_to_do,
        ));
    }
    Ok(())
}

/// What went wrong, and what `sv` itself says to do about it, kept apart. The MCP server fences what
/// went wrong as the app's text, since it quotes the app as often as not, and writes the next step
/// outside the fence as `sv`'s own words, which an AI coding tool reading a fenced instruction as
/// information did not act on (`docs/GAP-ANALYSIS.md`, 5.3). At a terminal the two read as one, as
/// before. `next` is `sv`'s words only: nothing of the app's goes in it.
#[derive(Debug)]
pub struct Remedy {
    pub problem: String,
    pub next: String,
}

impl Remedy {
    /// The error that carries `problem` and `next`.
    pub fn error(problem: impl Into<String>, next: impl Into<String>) -> anyhow::Error {
        anyhow::Error::new(Remedy {
            problem: problem.into(),
            next: next.into(),
        })
    }
}

impl std::fmt::Display for Remedy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.problem, self.next)
    }
}

impl std::error::Error for Remedy {}

/// Makes `relative` below `base` one folder at a time, refusing a level that is a link or is not a
/// folder before anything below it is made. `relative` holds only plain names and `.`, which the
/// caller has already checked. Gives the folder, and the ones it made, topmost first; refused part
/// way, it takes away the ones it made.
pub fn create_below(base: &Path, relative: &Path) -> Result<(PathBuf, Vec<PathBuf>)> {
    let mut made = Vec::new();
    let made_here = create_each(base, relative, &mut made);
    if made_here.is_err() {
        for folder in made.iter().rev() {
            let _ = std::fs::remove_dir(folder);
        }
    }
    made_here.map(|here| (here, made))
}

pub fn create_each(base: &Path, relative: &Path, made: &mut Vec<PathBuf>) -> Result<PathBuf> {
    let mut here = base.to_path_buf();
    for part in relative.components() {
        let Component::Normal(name) = part else {
            continue;
        };
        here.push(name);
        match std::fs::symlink_metadata(&here) {
            Ok(meta) if meta.file_type().is_symlink() => anyhow::bail!(
                "{} is a link to somewhere else, so nothing is written through it",
                here.display()
            ),
            Ok(meta) => {
                anyhow::ensure!(meta.is_dir(), "{} is not a folder", here.display());
            }
            Err(_) => {
                std::fs::create_dir(&here)
                    .with_context(|| format!("{} cannot be created", here.display()))?;
                made.push(here.clone());
            }
        }
    }
    Ok(here)
}

/// Turning off what the terminal shows while a passphrase is typed.
#[cfg(unix)]
pub mod quiet {
    pub const CAN_HIDE: bool = true;
    pub const CANNOT_HIDE: &str = "";

    /// What the terminal shows is put back however this returns.
    pub struct Shown(Option<libc::termios>);
    impl Drop for Shown {
        fn drop(&mut self) {
            if let Some(was) = self.0 {
                // SAFETY: `was` is the terminal's own settings, read below from the same descriptor.
                unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &was) };
            }
        }
    }

    pub fn hide_typing() -> Shown {
        let mut shown = Shown(None);
        // SAFETY: a zeroed termios is only written into by `tcgetattr`, and used only if that succeeds.
        let mut was: libc::termios = unsafe { std::mem::zeroed() };
        if unsafe { libc::tcgetattr(libc::STDIN_FILENO, &mut was) } == 0 {
            let mut quiet = was;
            quiet.c_lflag &= !libc::ECHO;
            // SAFETY: as above; `quiet` is the terminal's settings with echo turned off.
            if unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &quiet) } == 0 {
                shown.0 = Some(was);
            }
        }
        shown
    }
}

/// On Windows `sv` cannot yet ask the console to hide what is typed, so it says so instead of
/// pretending (backlog 0120, step 2).
#[cfg(not(unix))]
pub mod quiet {
    pub const CAN_HIDE: bool = false;
    pub const CANNOT_HIDE: &str = "(What you type next will show on the screen: StackVet cannot hide it on \
                                   this computer yet. Make sure nobody can see your screen.)";
    pub fn hide_typing() {}
}
