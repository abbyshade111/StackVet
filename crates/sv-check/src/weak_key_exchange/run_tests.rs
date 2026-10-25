//! The key-exchange reader runs with the other configuration checks (backlog 0625, part 12): a weak exchange in an
//! app's code is in the report `check_dir` makes, with the line it is on.

use super::*;

#[test]
fn the_configuration_checks_report_a_weak_exchange() {
    let dir = std::env::temp_dir().join(format!("sv-weak-exchange-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("server.js"),
        "const crypto = require('crypto');\nconst dh = crypto.getDiffieHellman('modp2');\n",
    )
    .unwrap();
    let report = crate::config::check_dir(&dir);
    std::fs::remove_dir_all(&dir).ok();
    let found: Vec<(&str, usize)> = report
        .findings
        .iter()
        .filter(|f| f.rule_id == WEAK_EXCHANGE)
        .map(|f| (f.location.file.as_str(), f.location.line))
        .collect();
    assert_eq!(
        found,
        [("server.js", 2)],
        "{:?}",
        report
            .findings
            .iter()
            .map(|f| &f.rule_id)
            .collect::<Vec<_>>()
    );
}
