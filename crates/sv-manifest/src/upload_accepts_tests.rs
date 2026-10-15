//! `accepts` under the `upload` entry: the kinds of file the owner says the app takes (backlog 0584, ADR-046, Later).

use super::*;

#[test]
fn accepts_is_read_in_its_own_order_and_left_out_is_not_said() {
    let base = "path = \"/upload\"\nfield = \"file\"\n";
    let u: UploadSection =
        toml::from_str(&format!("{base}accepts = [\".html\", \"image/png\"]\n")).unwrap();
    assert_eq!(
        u.accepts,
        Some(vec![".html".to_owned(), "image/png".to_owned()])
    );
    let u: UploadSection = toml::from_str(base).unwrap();
    assert_eq!(u.accepts, None);
    // A single string is not a list: refused, rather than read as one letter per kind.
    assert!(toml::from_str::<UploadSection>(&format!("{base}accepts = \"html\"\n")).is_err());
}
