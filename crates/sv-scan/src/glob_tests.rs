//! The one wildcard matcher (backlog 0456), held to what each of the two it replaced was tested for, and to the case
//! where they differed.

use super::matches as m;

#[test]
fn star_and_question_stay_within_a_name() {
    assert!(m("*.py", "app.py"));
    assert!(!m("*.py", "src/app.py"));
    assert!(m("a/?.py", "a/b.py"));
    assert!(!m("a/?.py", "a/bc.py"));
    assert!(!m("a?b", "a/b"));
}

#[test]
fn double_star_slash_is_whole_folders_or_none() {
    assert!(m(
        "**/sites-available/*",
        "etc/nginx/sites-available/default"
    ));
    assert!(m("**/sites-available/*", "sites-available/default"));
    assert!(!m("**/sites-available/*", "sites-available/old/default"));
    assert!(m(
        "**/wp-content/plugins/**/*.php",
        "wp-content/plugins/shop/shop.php"
    ));
    assert!(m(
        "**/wp-content/plugins/**/*.php",
        "site/wp-content/plugins/a.php"
    ));
    assert!(!m("**/wp-content/plugins/**/*.php", "src/plugins/a.php"));
    // Where the `.gitignore` matcher went wrong: `**/.env` read as `.env` anywhere, the end of a name included.
    assert!(m("**/.env", ".env"));
    assert!(m("**/.env", "deploy/prod/.env"));
    assert!(!m("**/.env", "my.env"));
    assert!(!m("**/.env", "deploy/my.env"));
}

#[test]
fn double_star_elsewhere_crosses_folders() {
    assert!(m("config/**", "config/a/b.yml"));
    assert!(m("a**b", "a/x/b"));
}

#[test]
fn a_set_and_an_escape_are_read_as_git_reads_them() {
    assert!(m(".env.[a-z]*", ".env.local"));
    assert!(!m(".env.[a-z]*", ".env.1"));
    assert!(m("[!.]env", "xenv"));
    assert!(!m("[!.]env", ".env"));
    assert!(m("[^.]env", "xenv"));
    assert!(!m("a[/]b", "a/b"), "a set never matches /");
    assert!(m("\\*.txt", "*.txt"));
    assert!(!m("\\*.txt", "a.txt"));
    // An unclosed `[` is itself.
    assert!(m("a[b", "a[b"));
}
