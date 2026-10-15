//! The kind of file the upload checks send (backlog 0584, ADR-046, Later).
//!
//! Every upload check is read beside an ordinary file the app took, so the ordinary file has to be one it takes. An app
//! that takes only HTML pages refuses a GIF, and then nothing can be judged. `accepts` under the `upload` entry says
//! what the app takes; the first kind on it that `sv` can make is the ordinary file, and the files built to be refused
//! (too large, the wrong contents, a name that climbs out of the folder) are that kind too, so a refusal can only be
//! about the one thing each changes. With no `accepts`, a GIF, as before.
//!
//! Each file is the kind's opening bytes, a marker, and its closing bytes: enough for a check of the first bytes, which
//! is what a check of an upload's contents reads, and no more. It is not a picture or a document a viewer would open.

/// A kind of file the upload checks can send.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::signed_in) struct Kind {
    /// The extension its files are named with.
    pub(in crate::signed_in) ext: &'static str,
    /// What one is called after "an ordinary": "GIF", "HTML page".
    pub(in crate::signed_in) short: &'static str,
    /// With its article, after "contents are not": "a GIF", "an HTML page".
    pub(in crate::signed_in) a: &'static str,
    /// The extensions and media types in `accepts` that name it, lower case.
    names: &'static [&'static str],
    head: &'static [u8],
    tail: &'static [u8],
    /// Contents that are not this kind, and a piece of text in them that shows they were kept.
    not_it: &'static [u8],
    pub(in crate::signed_in) not_it_holds: &'static str,
}

/// Contents that are not this kind for an image or a document: text, with a line of PHP in it.
macro_rules! not_an_image {
    ($what:literal) => {
        concat!(
            "<?php echo 'sv-probe'; ?>\nthis is not ",
            $what,
            " at all\n"
        )
        .as_bytes()
    };
}

/// Contents that are not this kind for text: an image's first bytes, with a NUL in them that no text holds.
const AN_IMAGE: &[u8] = b"GIF87a\x01\x00\x01\x00\x00\x00\x00sv-probe: an image, not text\x00;";

pub(in crate::signed_in) const GIF: Kind = Kind {
    ext: "gif",
    short: "GIF",
    a: "a GIF",
    names: &["gif", "image/gif", "image/*"],
    head: b"GIF87a",
    tail: b"",
    not_it: not_an_image!("a GIF"),
    not_it_holds: "this is not a GIF at all",
};

const PNG: Kind = Kind {
    ext: "png",
    short: "PNG image",
    a: "a PNG image",
    names: &["png", "image/png"],
    head: b"\x89PNG\r\n\x1a\n",
    tail: b"",
    not_it: not_an_image!("a PNG image"),
    not_it_holds: "this is not a PNG image at all",
};

const JPEG: Kind = Kind {
    ext: "jpg",
    short: "JPEG image",
    a: "a JPEG image",
    names: &["jpg", "jpeg", "image/jpeg"],
    head: b"\xff\xd8\xff\xe0",
    tail: b"\xff\xd9",
    not_it: not_an_image!("a JPEG image"),
    not_it_holds: "this is not a JPEG image at all",
};

const HTML: Kind = Kind {
    ext: "html",
    short: "HTML page",
    a: "an HTML page",
    names: &["html", "htm", "text/html"],
    head: b"<!doctype html>\n<html><head><title>sv-probe</title></head><body><p>",
    tail: b"</p></body></html>\n",
    not_it: AN_IMAGE,
    not_it_holds: "an image, not text",
};

const PDF: Kind = Kind {
    ext: "pdf",
    short: "PDF",
    a: "a PDF",
    names: &["pdf", "application/pdf"],
    head: b"%PDF-1.4\n%",
    tail: b"\n%%EOF\n",
    not_it: not_an_image!("a PDF"),
    not_it_holds: "this is not a PDF at all",
};

const TEXT: Kind = Kind {
    ext: "txt",
    short: "text file",
    a: "text",
    names: &["txt", "text/plain"],
    head: b"",
    tail: b"\n",
    not_it: AN_IMAGE,
    not_it_holds: "an image, not text",
};

const CSV: Kind = Kind {
    ext: "csv",
    short: "CSV file",
    a: "a CSV file",
    names: &["csv", "text/csv"],
    head: b"name,value\nsv-probe,",
    tail: b"\n",
    not_it: AN_IMAGE,
    not_it_holds: "an image, not text",
};

const JSON: Kind = Kind {
    ext: "json",
    short: "JSON file",
    a: "JSON",
    names: &["json", "application/json"],
    head: b"{\"sv-probe\": \"",
    tail: b"\"}\n",
    not_it: AN_IMAGE,
    not_it_holds: "an image, not text",
};

/// Every kind `sv` can make. Which is sent is decided by the order of `accepts`, not this one.
const KINDS: [&Kind; 8] = [&GIF, &PNG, &JPEG, &HTML, &PDF, &TEXT, &CSV, &JSON];

impl Kind {
    /// A file of this kind holding `marker`.
    pub(in crate::signed_in) fn file(&self, marker: &str) -> Vec<u8> {
        [self.head, marker.as_bytes(), self.tail].concat()
    }

    /// A file of this kind `pad` bytes of `A` long inside, and text that shows it was kept: the `A`s after the
    /// opening's last text, which is how a response read as text shows them.
    pub(in crate::signed_in) fn padded(&self, pad: usize) -> (Vec<u8>, String) {
        let file = [self.head, "A".repeat(pad).as_bytes(), self.tail].concat();
        let text = self
            .head
            .iter()
            .rposition(|b| !b.is_ascii())
            .map_or(self.head, |at| &self.head[at + 1..]);
        (file, format!("{}AAAAAAAA", String::from_utf8_lossy(text)))
    }

    /// How many bytes the opening and closing add to what goes inside.
    pub(in crate::signed_in) fn wrapping(&self) -> u64 {
        (self.head.len() + self.tail.len()) as u64
    }

    /// Contents that are not this kind.
    pub(in crate::signed_in) fn not_it(&self) -> Vec<u8> {
        self.not_it.to_vec()
    }
}

/// The kind the upload checks send for what `accepts` says: the first entry `sv` can make, or a GIF when `accepts` is
/// absent. `Err` says why none can be sent: the list is empty, or `sv` makes none of what it names.
pub(in crate::signed_in) fn chosen(accepts: Option<&[String]>) -> Result<&'static Kind, String> {
    let Some(accepts) = accepts else {
        return Ok(&GIF);
    };
    let named = |entry: &String| {
        let entry = entry.trim().to_ascii_lowercase();
        let entry = entry.strip_prefix('.').unwrap_or(&entry).to_owned();
        KINDS
            .into_iter()
            .find(|k| k.names.contains(&entry.as_str()))
    };
    if let Some(kind) = accepts.iter().find_map(named) {
        return Ok(kind);
    }
    let made = KINDS.map(|k| format!("`.{}`", k.ext)).join(", ");
    Err(if accepts.is_empty() {
        format!(
            "`accepts` under the `upload` entry is empty, so there is no kind of file to send. `sv` can send {made}."
        )
    } else {
        format!(
            "`accepts` under the `upload` entry names {}, and `sv` can send none of those: it can send {made}. Nothing \
             here can tell a file refused for being wrong from one refused for being a kind the app does not take.",
            accepts
                .iter()
                .map(|a| format!("`{}`", a.replace('`', "'")))
                .collect::<Vec<_>>()
                .join(", ")
        )
    })
}
