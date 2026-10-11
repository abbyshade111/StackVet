//! The helper images are named by digest as well as tag (backlog 0238), and the digest the run record keeps is read
//! from what Docker says, not from a tag.

use super::*;

#[test]
fn every_helper_image_is_named_by_a_tag_and_a_sha256_digest() {
    // The setup: the list has every helper the constants name.
    assert_eq!(HELPER_IMAGES.len(), 4, "{HELPER_IMAGES:?}");
    for image in HELPER_IMAGES {
        let (named, digest) = image
            .split_once("@sha256:")
            .unwrap_or_else(|| panic!("{image} is not named by digest"));
        assert!(
            named.contains(':'),
            "{image}: its tag is not kept beside the digest"
        );
        assert_eq!(digest.len(), 64, "{image}");
        assert!(
            digest
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()),
            "{image}"
        );
    }
}

#[test]
fn the_browser_version_is_still_read_from_its_tag_and_not_its_digest() {
    let tag = BROWSER_IMAGE
        .split('@')
        .next()
        .unwrap()
        .rsplit(':')
        .next()
        .unwrap();
    assert_eq!(tag, "151.0.7922.109");
}

#[test]
fn a_digest_is_read_from_a_repository_digest_or_an_image_id_and_nothing_else_is() {
    let hex = "73aaf090f3d85aa34ee199857f03fa3a95c8ede2ffd4cc2cdb5b94e566b11662";
    let sum = format!("sha256:{hex}");
    assert_eq!(digest_of(&format!("busybox@{sum}")), Some(sum.clone()));
    assert_eq!(digest_of(&sum), Some(sum.clone()));
    // Not a digest: a tag, a short or uppercase hex, the wrong algorithm, or nothing.
    assert_eq!(digest_of("busybox:1.36"), None);
    assert_eq!(digest_of("sha256:abc"), None);
    assert_eq!(digest_of(&format!("sha256:{}", hex.to_uppercase())), None);
    assert_eq!(digest_of(&format!("sha512:{hex}")), None);
    assert_eq!(digest_of(""), None);
}
