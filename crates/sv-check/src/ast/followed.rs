//! Following a name in a path to where the same file sets it (backlog 0582). In the Opus run of 15 October 2026,
//! `ast.file-path-from-value` reported three paths the app had made safe, and they were the app's only two "needs
//! attention" lines: a path kept in a module-level name, a file name made with `secrets.token_hex`, and a name checked
//! before use. Each passed through a name, and the rule judged the name's text, not its value.
//!
//! Python only, and only for a rule that asks (`safeNamesRead`). A name is safe when every value the file sets it to is
//! safe; a name the function takes as a parameter is safe only when a check before the call stops the function
//! otherwise. Anything this cannot read is not safe, so the rule reports it as before.

use super::*;

/// How many names deep a value is followed: `path = os.path.join(DIR, name)`, `name = token + ".png"`, and so on.
const DEPTH: usize = 4;

/// Whether `arg`, read through the names in it, is safe: fixed text, `pattern`, or a value the app made itself.
pub(super) fn safe_through_names(
    arg: tree_sitter::Node,
    source: &[u8],
    pattern: &regex::Regex,
    fixed: &Fixed,
) -> bool {
    safe(arg, arg.start_byte(), source, pattern, fixed, DEPTH)
}

fn text<'a>(node: tree_sitter::Node, source: &'a [u8]) -> &'a str {
    node.utf8_text(source).unwrap_or("")
}

fn named(node: tree_sitter::Node) -> Vec<tree_sitter::Node> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .filter(|c| c.kind() != "comment")
        .collect()
}

/// A value nobody outside the app chose: `secrets.token_hex(16)`, `uuid.uuid4()`, and what is read off one
/// (`uuid.uuid4().hex`, `str(uuid.uuid4())`).
fn generated(node: tree_sitter::Node, source: &[u8]) -> bool {
    match node.kind() {
        "call" => {
            let function = node
                .child_by_field_name("function")
                .map(|f| text(f, source))
                .unwrap_or("");
            if matches!(
                function,
                "secrets.token_hex"
                    | "secrets.token_urlsafe"
                    | "token_hex"
                    | "token_urlsafe"
                    | "uuid.uuid4"
                    | "uuid.uuid1"
                    | "uuid4"
                    | "uuid1"
            ) {
                return true;
            }
            // `str(uuid.uuid4())`
            function == "str"
                && node
                    .child_by_field_name("arguments")
                    .map(named)
                    .is_some_and(|args| args.len() == 1 && generated(args[0], source))
        }
        "attribute" => node
            .child_by_field_name("object")
            .is_some_and(|object| generated(object, source)),
        _ => false,
    }
}

fn safe(
    node: tree_sitter::Node,
    at: usize,
    source: &[u8],
    pattern: &regex::Regex,
    fixed: &Fixed,
    depth: usize,
) -> bool {
    if is_literal(node, source, fixed) || generated(node, source) {
        return true;
    }
    match node.kind() {
        // The app's own file.
        "identifier" if text(node, source) == "__file__" => true,
        "identifier" => depth > 0 && name_safe(node, at, source, pattern, fixed, depth - 1),
        "parenthesized_expression" => named(node)
            .into_iter()
            .all(|c| safe(c, at, source, pattern, fixed, depth)),
        "binary_operator"
            if node
                .child_by_field_name("operator")
                .is_some_and(|o| text(o, source) == "+") =>
        {
            named(node)
                .into_iter()
                .all(|c| safe(c, at, source, pattern, fixed, depth))
        }
        // An f-string: each `{…}` is judged; the text around them is fixed.
        "string" => named(node)
            .into_iter()
            .filter(|c| c.kind() == "interpolation")
            .filter_map(|c| c.child_by_field_name("expression"))
            .all(|e| safe(e, at, source, pattern, fixed, depth)),
        // `os.path.join(a, b)`, and the app folder's own `os.path.dirname(os.path.abspath(__file__))`: safe when every
        // part is.
        "call"
            if node.child_by_field_name("function").is_some_and(|f| {
                matches!(
                    text(f, source),
                    "os.path.join" | "os.path.dirname" | "os.path.abspath" | "os.path.realpath"
                )
            }) =>
        {
            node.child_by_field_name("arguments")
                .map(named)
                .is_some_and(|args| {
                    !args.is_empty()
                        && args
                            .into_iter()
                            .all(|a| safe(a, at, source, pattern, fixed, depth))
                })
        }
        // Anything else is safe only as the rule's own pattern says.
        _ => pattern.is_match(text(node, source)),
    }
}

/// The function `node` is in, or the module.
fn scope_of(node: tree_sitter::Node) -> tree_sitter::Node {
    let mut scope = node;
    while let Some(parent) = scope.parent() {
        scope = parent;
        if scope.kind() == "function_definition" {
            break;
        }
    }
    scope
}

/// Every value `name` is set to under `scope`, or `None` when it is set some way this does not read: `+=`, a loop, a
/// `with … as`, an `except … as`, a `global` or `nonlocal`, or a walrus.
fn values_set<'t>(
    scope: tree_sitter::Node<'t>,
    name: &str,
    source: &[u8],
) -> Option<Vec<tree_sitter::Node<'t>>> {
    let mut values = Vec::new();
    let mut stack = vec![scope];
    while let Some(node) = stack.pop() {
        match node.kind() {
            "assignment" => {
                let target = node.child_by_field_name("left").map(|l| text(l, source));
                if target == Some(name) {
                    values.push(node.child_by_field_name("right")?);
                } else if target.is_some_and(|t| {
                    t.split(|c: char| !(c.is_alphanumeric() || c == '_'))
                        .any(|w| w == name)
                }) && !target.is_some_and(|t| t.contains('.') || t.contains('['))
                {
                    // `a, name = …`: set some way this does not read.
                    return None;
                }
            }
            "augmented_assignment"
            | "named_expression"
            | "for_statement"
            | "as_pattern"
            | "global_statement"
            | "nonlocal_statement"
                if named(node)
                    .iter()
                    .any(|c| c.kind() == "identifier" && text(*c, source) == name)
                    || node
                        .child_by_field_name("left")
                        .or_else(|| node.child_by_field_name("name"))
                        .is_some_and(|l| text(l, source) == name)
                    || (node.kind() == "as_pattern"
                        && node
                            .child_by_field_name("alias")
                            .is_some_and(|a| text(a, source).trim() == name)) =>
            {
                return None;
            }
            _ => {}
        }
        stack.extend(named(node));
    }
    Some(values)
}

/// Whether `function` takes `name` as a parameter.
fn takes(function: tree_sitter::Node, name: &str, source: &[u8]) -> bool {
    function
        .child_by_field_name("parameters")
        .map(named)
        .unwrap_or_default()
        .into_iter()
        .any(|p| {
            let own = match p.kind() {
                "identifier" => Some(p),
                _ => p
                    .child_by_field_name("name")
                    .or_else(|| named(p).into_iter().find(|c| c.kind() == "identifier")),
            };
            own.is_some_and(|o| text(o, source) == name)
        })
}

fn name_safe(
    node: tree_sitter::Node,
    at: usize,
    source: &[u8],
    pattern: &regex::Regex,
    fixed: &Fixed,
    depth: usize,
) -> bool {
    let name = text(node, source);
    let scope = scope_of(node);
    if scope.kind() == "function_definition" {
        let Some(local) = values_set(scope, name, source) else {
            return false;
        };
        if takes(scope, name, source) {
            // A parameter is what the caller passed: safe only when checked first, and never set again.
            return local.is_empty() && checked_before(scope, name, at, source);
        }
        if !local.is_empty() {
            return local
                .into_iter()
                .all(|v| safe(v, at, source, pattern, fixed, depth));
        }
    }
    // Not the function's own: the module's, read in the whole file, so a function that sets it too is counted.
    let mut root = node;
    while let Some(parent) = root.parent() {
        root = parent;
    }
    match values_set(root, name, source) {
        Some(values) if !values.is_empty() => values
            .into_iter()
            .all(|v| safe(v, usize::MAX, source, pattern, fixed, depth)),
        _ => false,
    }
}

/// Whether, before `at` and straight in `function`'s body, an `if` stops the function unless `name` passed a narrow
/// check: `if not re.fullmatch(r"[a-z0-9_-]+", name): abort(400)`, or `if secure_filename(name) != name: …`.
fn checked_before(function: tree_sitter::Node, name: &str, at: usize, source: &[u8]) -> bool {
    let Some(body) = function.child_by_field_name("body") else {
        return false;
    };
    named(body).into_iter().any(|statement| {
        statement.kind() == "if_statement"
            && statement.end_byte() <= at
            && statement
                .child_by_field_name("condition")
                .is_some_and(|c| refuses_unless_narrow(c, name, source))
            && statement
                .child_by_field_name("consequence")
                .and_then(|block| named(block).last().copied())
                .is_some_and(|last| stops(last, source))
            && statement.child_by_field_name("alternative").is_none()
    })
}

/// A statement that leaves the function: `raise`, `return`, or `abort(...)`.
fn stops(statement: tree_sitter::Node, source: &[u8]) -> bool {
    match statement.kind() {
        "raise_statement" | "return_statement" => true,
        "expression_statement" => named(statement).first().is_some_and(|e| {
            e.kind() == "call"
                && e.child_by_field_name("function")
                    .is_some_and(|f| matches!(text(f, source), "abort" | "flask.abort"))
        }),
        _ => false,
    }
}

/// `not re.fullmatch(P, name)` (or `re.match` with `P` ending in `$`) with a narrow `P`, or `secure_filename(name) !=
/// name` either way round.
fn refuses_unless_narrow(condition: tree_sitter::Node, name: &str, source: &[u8]) -> bool {
    let whole = text(condition, source)
        .split_whitespace()
        .collect::<String>();
    if whole == format!("secure_filename({name})!={name}")
        || whole == format!("{name}!=secure_filename({name})")
    {
        return true;
    }
    if condition.kind() != "not_operator" {
        return false;
    }
    let Some(call) = condition.child_by_field_name("argument") else {
        return false;
    };
    let function = call
        .child_by_field_name("function")
        .map(|f| text(f, source))
        .unwrap_or("");
    let args = call
        .child_by_field_name("arguments")
        .map(named)
        .unwrap_or_default();
    let [regex, checked] = args.as_slice() else {
        return false;
    };
    if text(*checked, source) != name || regex.kind() != "string" {
        return false;
    }
    let written = text(*regex, source);
    let body = written
        .trim_start_matches(|c: char| c.is_ascii_alphabetic())
        .trim_matches(|c| c == '"' || c == '\'');
    let anchored = function == "re.fullmatch"
        || (function == "re.match" && (body.ends_with('$') || body.ends_with("\\Z")));
    // Narrow: nothing that could let a `.` or a `/` through.
    let narrow = !body.is_empty()
        && !body.contains('.')
        && !body.contains('/')
        && !body.contains("[^")
        && !["\\S", "\\W", "\\D", "\\s"]
            .iter()
            .any(|wide| body.contains(wide));
    anchored && narrow
}
