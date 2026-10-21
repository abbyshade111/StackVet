//! V11.2.5 (a cryptographic module fails securely) cited by `ast.check-passes-on-error` only where the code its error
//! handler guards is a signature check, a MAC check, or a decryption (backlog 0507): the narrower match `alsoCites`
//! names. Everywhere else the finding cites V16.5.3 alone, and the loader refuses a narrower citation it cannot make.

use super::*;
use std::path::PathBuf;

fn rules() -> &'static AstRules {
    static RULES: std::sync::OnceLock<AstRules> = std::sync::OnceLock::new();
    RULES.get_or_init(|| {
        AstRules::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"))
            .expect("rules load")
    })
}

/// What the one `ast.check-passes-on-error` finding in `source` cites.
fn cited(language: &str, source: &str) -> Vec<String> {
    let found: Vec<Finding> = scan_file(rules(), language, &format!("src/app.{language}"), source)
        .into_iter()
        .filter(|f| f.rule_id == "ast.check-passes-on-error")
        .collect();
    assert_eq!(found.len(), 1, "{source}: {found:?}");
    found[0].requirement_ids.clone()
}

const BOTH: [&str; 2] = ["V16.5.3", "V11.2.5"];
const ALONE: [&str; 1] = ["V16.5.3"];

#[test]
fn a_signature_check_or_decryption_that_fails_open_cites_v11_2_5_too() {
    assert_eq!(
        cited(
            "python",
            "def verify_token(t):\n    try:\n        jwt.decode(t, KEY, algorithms=['HS256'])\n        return True\n    except Exception:\n        return True\n"
        ),
        BOTH
    );
    assert_eq!(
        cited(
            "python",
            "def verify_cookie(c):\n    try:\n        return box.decrypt(c)\n    except InvalidToken:\n        return True\n"
        ),
        BOTH
    );
    assert_eq!(
        cited(
            "javascript",
            "function verifyToken(t) { try { jwt.verify(t, KEY); return true; } catch (e) { return true; } }"
        ),
        BOTH
    );
    assert_eq!(
        cited(
            "typescript",
            "async function verifySignature(k: CryptoKey, s: ArrayBuffer, d: ArrayBuffer): Promise<boolean> { try { return await crypto.subtle.verify('HMAC', k, s, d); } catch (e) { return true; } }"
        ),
        BOTH
    );
}

#[test]
fn any_other_check_that_fails_open_cites_v16_5_3_alone() {
    // The setup is the same shape as above, so the difference is what the `try` guards.
    assert_eq!(
        cited(
            "python",
            "class Perms:\n    @staticmethod\n    def has_permission(user, thing):\n        try:\n            return lookup(user, thing)\n        except KeyError:\n            return True\n"
        ),
        ALONE
    );
    // A decryption in the handler itself is not one the handler answers for.
    assert_eq!(
        cited(
            "python",
            "def check_access(u):\n    try:\n        return acl.allowed(u)\n    except Exception:\n        audit.decrypt(u)\n        return True\n"
        ),
        ALONE
    );
    assert_eq!(
        cited(
            "javascript",
            "exports.isAllowed = async (u, r) => { try { return await acl.check(u, r); } catch (e) { return true; } };"
        ),
        ALONE
    );
    // Go has no `try`: its `if err != nil` is never read as guarding anything, so it is never cited more narrowly.
    assert_eq!(
        cited(
            "go",
            "package m\nfunc verifyToken(t string) bool { _, err := jwt.Parse(t, keyFunc); if err != nil { return true }; return true }"
        ),
        ALONE
    );
}

/// The shipped rules with `edit` made to `ast.check-passes-on-error`'s `alsoCites`, loaded from a scratch file.
fn load_with(edit: impl Fn(&mut serde_json::Value)) -> Result<AstRules> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json");
    let mut data: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    let rule = data["rules"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["id"] == "ast.check-passes-on-error")
        .expect("the rule is shipped");
    edit(&mut rule["alsoCites"][0]);
    let dir = crate::test_support::scratch("");
    let file = dir.join("ast-rules.json");
    std::fs::write(&file, serde_json::to_string(&data).unwrap()).unwrap();
    AstRules::load(&file)
}

#[test]
fn a_narrower_citation_that_cannot_be_made_is_refused_when_the_rules_load() {
    // The setup: unchanged, the scratch copy loads.
    assert!(load_with(|_| {}).is_ok());
    let refused = |edit: &dyn Fn(&mut serde_json::Value), said: &str| {
        let e = load_with(edit)
            .err()
            .map(|e| format!("{e:#}"))
            .unwrap_or_default();
        assert!(e.contains(said), "expected {said:?}, got {e:?}");
    };
    refused(
        &|c| c["requirementIds"] = serde_json::json!(["V16.5.3"]),
        "always and on a narrower match",
    );
    refused(
        &|c| c["guardedPatterns"]["elixir"] = serde_json::json!("x"),
        "no elixir query",
    );
    refused(
        &|c| c["why"] = serde_json::json!(" "),
        "without saying what, or why",
    );
}
