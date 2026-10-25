//! Rust's safe forms for the path rules (backlog 0653; ADR-138, Later): a path every part of which is fixed in the app's
//! own code, as `followed.rs` reads Python's. A path is fixed when it is written out (a string, a raw string), a
//! constant (`ROOT`, as the other languages' patterns read a name in capitals), a compile-time macro
//! (`env!("CARGO_MANIFEST_DIR")`), `Path::new` or `PathBuf::from` of a fixed path, a fixed path's `.join` (and the
//! methods like it) of fixed paths, a reference to one, or a name the function sets only to fixed paths and never sets
//! again. Anything else, a parameter among them, is not, so the call is judged as before.

use super::fixed::{Fixed, compile_time_text, is_constant_name, is_literal};

/// How many names deep a path is followed, as `followed.rs` follows Python's.
const DEPTH: usize = 3;

/// The methods that make a path from a path and their arguments, and nothing else.
const PATH_METHODS: &[&str] = &[
    "join",
    "with_extension",
    "with_file_name",
    "to_path_buf",
    "to_owned",
    "as_path",
    "as_ref",
    "into",
    "clone",
];

/// The functions that make a path from their argument alone.
const PATH_MAKERS: &[&str] = &[
    "Path::new",
    "PathBuf::from",
    "std::path::Path::new",
    "std::path::PathBuf::from",
];

pub(super) fn fixed_path(arg: tree_sitter::Node, source: &[u8], fixed: &Fixed) -> bool {
    path(arg, source, fixed, DEPTH)
}

fn text<'a>(node: tree_sitter::Node, source: &'a [u8]) -> &'a str {
    node.utf8_text(source).unwrap_or("")
}

fn named(node: tree_sitter::Node) -> Vec<tree_sitter::Node> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .filter(|c| !matches!(c.kind(), "line_comment" | "block_comment"))
        .collect()
}

fn path(node: tree_sitter::Node, source: &[u8], fixed: &Fixed, depth: usize) -> bool {
    match node.kind() {
        "string_literal" | "raw_string_literal" => true,
        "reference_expression" | "parenthesized_expression" => node
            .child_by_field_name("value")
            .or_else(|| named(node).into_iter().next())
            .is_some_and(|v| path(v, source, fixed, depth)),
        "macro_invocation" => compile_time_text(node, source),
        "identifier" => {
            let name = text(node, source);
            is_constant_name(name)
                || is_literal(node, source, fixed)
                || (depth > 0 && name_fixed(node, name, source, fixed, depth - 1))
        }
        // `config::ROOT`: a constant named by its path.
        "scoped_identifier" => node
            .child_by_field_name("name")
            .is_some_and(|n| is_constant_name(text(n, source))),
        "call_expression" => {
            let Some(function) = node.child_by_field_name("function") else {
                return false;
            };
            let arguments = node
                .child_by_field_name("arguments")
                .map(named)
                .unwrap_or_default();
            let all_fixed =
                |args: &[tree_sitter::Node]| args.iter().all(|a| path(*a, source, fixed, depth));
            match function.kind() {
                "scoped_identifier" if PATH_MAKERS.contains(&text(function, source)) => {
                    arguments.len() == 1 && all_fixed(&arguments)
                }
                // `base.join("data")`: the path it starts from, and every argument, fixed.
                "field_expression" => {
                    function
                        .child_by_field_name("field")
                        .is_some_and(|f| PATH_METHODS.contains(&text(f, source)))
                        && function
                            .child_by_field_name("value")
                            .is_some_and(|v| path(v, source, fixed, depth))
                        && all_fixed(&arguments)
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// The function `node` is in, or the file.
fn scope_of(node: tree_sitter::Node) -> tree_sitter::Node {
    let mut scope = node;
    while let Some(parent) = scope.parent() {
        scope = parent;
        if matches!(scope.kind(), "function_item" | "closure_expression") {
            break;
        }
    }
    scope
}

/// Whether every `let` of `name` in its function sets it to a fixed path, and nothing sets it again: at least one
/// such `let`, no assignment or compound assignment to it, and no `let` that binds it some way this does not read
/// (in a tuple, a pattern, or with no value).
fn name_fixed(
    node: tree_sitter::Node,
    name: &str,
    source: &[u8],
    fixed: &Fixed,
    depth: usize,
) -> bool {
    let scope = scope_of(node);
    let mut values = Vec::new();
    let mut stack = vec![scope];
    while let Some(n) = stack.pop() {
        match n.kind() {
            "let_declaration" => {
                let pattern = n.child_by_field_name("pattern");
                let simple =
                    pattern.is_some_and(|p| p.kind() == "identifier" && text(p, source) == name);
                let mentions = pattern.is_some_and(|p| {
                    text(p, source)
                        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
                        .any(|w| w == name)
                });
                if simple {
                    match n.child_by_field_name("value") {
                        Some(value) => values.push(value),
                        None => return false,
                    }
                } else if mentions {
                    return false;
                }
            }
            "assignment_expression" | "compound_assignment_expr"
                if n.child_by_field_name("left")
                    .is_some_and(|l| text(l, source) == name) =>
            {
                return false;
            }
            // A parameter of the function by this name: a value the caller chose.
            "parameter"
                if n.child_by_field_name("pattern")
                    .is_some_and(|p| text(p, source) == name) =>
            {
                return false;
            }
            _ => {}
        }
        stack.extend(named(n));
    }
    !values.is_empty() && values.into_iter().all(|v| path(v, source, fixed, depth))
}
