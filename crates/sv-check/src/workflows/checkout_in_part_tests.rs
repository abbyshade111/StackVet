//! AC.12.2 asks that pipeline tokens not be kept in a workspace that runs untrusted code (the checkout's
//! `persist-credentials: false`) and that runners be scrubbed of cached credentials before AI tooling
//! runs, which no workflow file shows. A clean reading of the checkouts is its first half, *checked in
//! part* (backlog 0469; ADR-053).

use super::*;

const SAFE: &str = "\
name: CI
on: [push, pull_request]
permissions:
  contents: read
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          persist-credentials: false
      - run: npm test
";

#[test]
fn every_checkout_dropping_its_token_credits_ac_12_2_in_part() {
    let dir =
        std::env::temp_dir().join(format!("sv-workflows-checkout-part-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join(".github/workflows")).unwrap();
    std::fs::write(dir.join(".github/workflows/ci.yml"), SAFE).unwrap();
    let report = check(&dir);
    std::fs::remove_dir_all(&dir).ok();
    let credit = report
        .passed
        .iter()
        .find(|v| v.check_id == CHECKOUT_TOKEN && v.requirement_ids == ["AC.12.2"])
        .unwrap_or_else(|| panic!("the setup: no AC.12.2 credit: {:#?}", report.passed));
    assert!(credit.in_part, "{credit:?}");
}
