//! Backlog 0643, ADR-135: in Rust and Go, a program started with its arguments as a list runs no shell, so
//! `Command::new(value)` and `exec.Command(value, ...)` are not a shell command built from a value. A program given
//! `-c` and then a value is a shell, however it is named, and still is.

use super::*;
use std::path::PathBuf;

fn shell_command_in(language: &str, source: &str) -> bool {
    static RULES: std::sync::OnceLock<AstRules> = std::sync::OnceLock::new();
    let rules = RULES.get_or_init(|| {
        AstRules::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"))
            .expect("rules load")
    });
    scan_file(rules, language, &format!("src/app.{language}"), source)
        .iter()
        .any(|f| f.rule_id == "ast.shell-command")
}

#[test]
fn a_program_given_its_arguments_as_a_list_is_not_a_shell_command() {
    for (language, source) in [
        (
            "rust",
            "fn f(program: &Path) { let mut c = Command::new(program); c.args(&args); }",
        ),
        (
            "rust",
            "fn f(a: &[String]) { std::process::Command::new(curl_program()).args(self.args(a)).output(); }",
        ),
        (
            "rust",
            "fn f(b: &str) { Command::new(&self.binary).args([\"exec\", b]).output(); }",
        ),
        (
            "rust",
            "fn f(exe: &Path) { Command::new(exe).arg(\"report\").arg(b).status(); }",
        ),
        (
            "go",
            "func f(name, b string) { exec.Command(name, b).Run() }",
        ),
        (
            "go",
            "func f(ctx context.Context, name, b string) { exec.CommandContext(ctx, name, b).Run() }",
        ),
    ] {
        assert!(
            !shell_command_in(language, source),
            "{language}: `{source}` was reported"
        );
    }
}

#[test]
fn a_program_given_dash_c_and_a_value_is_a_shell_however_it_is_named() {
    for (language, source) in [
        (
            "rust",
            "fn f(shell: &str, c: &str) { Command::new(shell).arg(\"-c\").arg(c).output(); }",
        ),
        (
            "rust",
            "fn f(shell: &str, c: &str) { Command::new(shell).args([\"-c\", c]).output(); }",
        ),
        (
            "rust",
            "fn f(shell: &str, c: &str) { Command::new(shell).args(&[\"-c\", c]).output(); }",
        ),
        (
            "rust",
            "fn f(c: &str) { Command::new(\"sh\").arg(\"-c\").arg(c).output(); }",
        ),
        (
            "go",
            "func f(shell, c string) { exec.Command(shell, \"-c\", c).Run() }",
        ),
        (
            "go",
            "func f(ctx context.Context, shell, c string) { exec.CommandContext(ctx, shell, \"-c\", c).Run() }",
        ),
        (
            "go",
            "func f(c string) { exec.Command(\"sh\", \"-c\", c).Run() }",
        ),
    ] {
        assert!(
            shell_command_in(language, source),
            "{language}: `{source}` was missed"
        );
    }
    // Written out in full, `-c`'s command is fixed text, and runs nothing a value chose.
    for (language, source) in [
        (
            "rust",
            "fn f(shell: &str) { Command::new(shell).arg(\"-c\").arg(\"ls -la\").output(); }",
        ),
        (
            "rust",
            "fn f(shell: &str) { Command::new(shell).args([\"-c\", \"date\"]).output(); }",
        ),
        (
            "go",
            "func f(shell string) { exec.Command(shell, \"-c\", \"date\").Run() }",
        ),
    ] {
        assert!(
            !shell_command_in(language, source),
            "{language}: `{source}` was reported"
        );
    }
}
