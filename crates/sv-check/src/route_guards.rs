//! The app's routes and the guards written on each, as the code declares them, for the authorization questions
//! (backlog 0493; ADR-022, Later).
//!
//! V8.1.1 and V8.1.2 ask the owner whether the app's rules for who may do what are written down. In the Antigravity
//! trial the code offered as proof showed two routes with no check at all (backlog 0397). This lists every route `sv`
//! can read, with what is written on it: a decorator such as `@login_required`, a FastAPI `Depends(...)`, or the
//! functions an Express route runs before its handler. A person who is not a programmer can hold "none found in this
//! file" against their own rules.
//!
//! # It finds, and does not judge
//!
//! Like a locator (`crate::locate`), this makes no finding and credits nothing. A guard named here may check nothing,
//! and a route with "none found" may be guarded by something this does not see: a check applied to every route at
//! once (`app.use(...)`, `before_request`), a check inside the handler, or a framework this does not read yet. So the
//! table is where to look, never evidence, and the owner's answer is credited at the tier of whoever gave it.
//!
//! # What it reads
//!
//! - Python: a function under a route decorator (`@app.route`, `@app.get`, `@router.post`, and the rest), the other
//!   decorators on it, and `Depends(...)` or `Security(...)` in its parameters or the decorator's `dependencies=`.
//! - JavaScript and TypeScript: `app.get('/path', ...)` and the like on an object named as an app or a router, with
//!   every argument between the path and the last as a guard.
//! - Django: each `path(...)` in a `urls.py`, with the decorators, mixins, and `permission_classes` on its view
//!   wherever the view is written, and any wrapper around it in `urls.py` (backlog 0608).
//! - Go: `http.HandleFunc`, `mux.Handle`, and router calls such as chi's and gin's, with the wrappers around the
//!   handler, or every handler but the last, as guards (backlog 0608).
//! - A check applied to every route at once, as its own row (`EVERY ROUTE`): `app.use(...)` and `r.Use(...)` when the
//!   middleware's name says it is a check, and every `before_request` (backlog 0608).
//!
//! Test code is left out: a test's requests are not the app's routes.

use std::collections::BTreeSet;
use tree_sitter::{Node, Parser};

/// The most routes a question shows; the rest are counted.
pub const MOST_ROUTES: usize = 40;

/// The most characters of one guard shown.
const GUARD_CHARS: usize = 80;

/// What the table is, and what it is not, shown above it.
pub const ROUTES_SAY: &str = "The routes StackVet could read, and what is written on each to decide who may use it, as \
     the code says it. These are where to look, not evidence: a check written on a route may check nothing, and \
     \"none found\" may be covered by a check inside the handler, or by a way of declaring routes StackVet does not \
     read yet. It reads Python decorators (Flask, FastAPI), Django's urls.py with its views' decorators and mixins, \
     Express-style `app.get('/path', ...)`, and Go's handlers; a check applied to every route at once \
     (`app.use(requireAuth)`, `before_request`, `r.Use(...)`) is its own row, EVERY ROUTE, when its name says it is \
     one.";

/// What a route with nothing written on it shows.
pub const NONE_FOUND: &str = "none found in this file";

/// The HTTP methods a route can be declared for, as the frameworks spell the function.
const METHODS: &[&str] = &["get", "post", "put", "patch", "delete", "head", "options"];

/// One route, as declared.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteLine {
    pub file: String,
    pub line: usize,
    /// `GET`, `POST`, or several joined by `, `.
    pub method: String,
    pub path: String,
    /// What is written on it, as the code says it; empty when nothing is.
    pub guards: Vec<String>,
}

/// Every route found in an app, in file and line order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RouteTable {
    pub routes: Vec<RouteLine>,
    /// Routes past `MOST_ROUTES`, counted and not kept.
    pub more: usize,
}

/// The routes in the app's own files, not counting test code, the first `MOST_ROUTES` kept.
pub fn read(listing: &sv_scan::files::Listing) -> RouteTable {
    let mut routes = read_all(listing);
    let more = routes.len().saturating_sub(MOST_ROUTES);
    routes.truncate(MOST_ROUTES);
    RouteTable { routes, more }
}

/// Every route in the app's own files, not counting test code, in file and line order.
pub fn read_all(listing: &sv_scan::files::Listing) -> Vec<RouteLine> {
    let mut routes = Vec::new();
    let mut python_files: Vec<(String, String)> = Vec::new();
    for entry in listing.app_files() {
        let Some(language) = entry.language else {
            continue;
        };
        if sv_scan::files::is_test_path(&entry.relative) {
            continue;
        }
        let key = match language {
            "python" => "python",
            "go" => "go",
            "javascript" => "javascript",
            "typescript" if entry.relative.to_lowercase().ends_with(".tsx") => "tsx",
            "typescript" => "typescript",
            _ => continue,
        };
        let Ok(source) = entry.read_text() else {
            continue;
        };
        if key == "python" {
            python_files.push((entry.relative.clone(), source.clone()));
        }
        // Most files declare no route, and are not parsed for one.
        if !METHODS.iter().any(|m| source.contains(m))
            && !["route", "Handle", "use(", "Use(", "before_"]
                .iter()
                .any(|w| source.contains(w))
        {
            continue;
        }
        routes.extend(in_source(key, &entry.relative, &source));
    }
    // Django: each `urls.py` route, with what is written on its view wherever the view is (backlog 0608).
    let urls: Vec<&(String, String)> = python_files
        .iter()
        .filter(|(file, _)| file.rsplit('/').next() == Some("urls.py"))
        .collect();
    if !urls.is_empty() {
        let mut views = std::collections::BTreeMap::new();
        for (_, source) in &python_files {
            for (name, guards) in wider::python_view_guards(source) {
                views.entry(name).or_insert(guards);
            }
        }
        for (file, source) in urls {
            routes.extend(wider::django_routes(file, source, &views));
        }
        routes.sort_by(|a, b| a.file.cmp(&b.file).then(a.line.cmp(&b.line)));
    }
    routes
}

/// The routes in one file, in the grammar `key` names.
pub fn in_source(key: &str, file: &str, source: &str) -> Vec<RouteLine> {
    let Some(grammar) = crate::ast::grammar(key) else {
        return Vec::new();
    };
    let mut parser = Parser::new();
    if parser.set_language(&grammar).is_err() {
        return Vec::new();
    }
    let Some(tree) = parser.parse(source, None) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    walk(tree.root_node(), &mut |node| {
        let found = match key {
            "python" => {
                python_route(node, source).or_else(|| wider::python_every_route(node, source))
            }
            "go" => wider::go_route(node, source),
            _ => script_route(node, source).or_else(|| wider::script_every_route(node, source)),
        };
        if let Some((line, method, path, guards)) = found
            && seen.insert((line, method.clone(), path.clone()))
        {
            out.push(RouteLine {
                file: file.to_owned(),
                line,
                method,
                path,
                guards,
            });
        }
    });
    out
}

fn walk<'a>(node: Node<'a>, visit: &mut impl FnMut(Node<'a>)) {
    visit(node);
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk(child, visit);
    }
}

fn text<'a>(node: Node, source: &'a str) -> &'a str {
    node.utf8_text(source.as_bytes()).unwrap_or("")
}

/// A guard as shown: on one line, and cut to a length a person reads.
fn shown(guard: &str) -> String {
    let one_line = guard.split_whitespace().collect::<Vec<_>>().join(" ");
    crate::text::cut_with_count(&one_line, GUARD_CHARS)
}

/// A string literal's text without its quotes or prefix; `None` when the node is not a plain string.
fn literal(node: Node, source: &str) -> Option<String> {
    if !matches!(node.kind(), "string" | "template_string") {
        return None;
    }
    let raw = text(node, source);
    if raw.contains("${") || node.kind() == "string" && raw.starts_with(['f', 'F']) {
        return None;
    }
    let body = raw.trim_start_matches(|c: char| c.is_ascii_alphabetic());
    let quote = body.chars().next()?;
    let inner = body.strip_prefix(quote)?.strip_suffix(quote)?;
    Some(inner.to_owned())
}

/// A Python function under a route decorator: its line, methods, path, and the other decorators and dependencies.
fn python_route(node: Node, source: &str) -> Option<(usize, String, String, Vec<String>)> {
    if node.kind() != "decorated_definition" {
        return None;
    }
    let definition = node.child_by_field_name("definition")?;
    if definition.kind() != "function_definition" {
        return None;
    }
    let mut route = None;
    let mut guards = Vec::new();
    let mut cursor = node.walk();
    for decorator in node.children(&mut cursor) {
        if decorator.kind() != "decorator" {
            continue;
        }
        let Some(expression) = decorator.named_child(0) else {
            continue;
        };
        match python_route_decorator(expression, source) {
            Some((method, path, dependencies)) if route.is_none() => {
                route = Some((decorator.start_position().row + 1, method, path));
                guards.extend(dependencies);
            }
            _ => guards.push(shown(text(expression, source))),
        }
    }
    let (line, method, path) = route?;
    if let Some(parameters) = definition.child_by_field_name("parameters") {
        let mut cursor = parameters.walk();
        for parameter in parameters.named_children(&mut cursor) {
            if let Some(value) = parameter.child_by_field_name("value") {
                let value = text(value, source);
                if value.starts_with("Depends(") || value.starts_with("Security(") {
                    guards.push(shown(value));
                }
            }
        }
    }
    Some((line, method, path, guards))
}

/// `app.route("/x", methods=[...])`, `router.get("/x", dependencies=[...])`: its methods, its path, and what its
/// `dependencies=` names.
fn python_route_decorator(expression: Node, source: &str) -> Option<(String, String, Vec<String>)> {
    if expression.kind() != "call" {
        return None;
    }
    let function = expression.child_by_field_name("function")?;
    if function.kind() != "attribute" {
        return None;
    }
    let name = text(function.child_by_field_name("attribute")?, source);
    let arguments = expression.child_by_field_name("arguments")?;
    let path = literal(arguments.named_child(0)?, source)?;
    if !path.starts_with('/') {
        return None;
    }
    let mut methods = Vec::new();
    let mut dependencies = Vec::new();
    let mut cursor = arguments.walk();
    for argument in arguments.named_children(&mut cursor) {
        if argument.kind() != "keyword_argument" {
            continue;
        }
        let (Some(key), Some(value)) = (
            argument.child_by_field_name("name"),
            argument.child_by_field_name("value"),
        ) else {
            continue;
        };
        match text(key, source) {
            "methods" => {
                let mut cursor = value.walk();
                methods.extend(
                    value
                        .named_children(&mut cursor)
                        .filter_map(|m| literal(m, source))
                        .map(|m| m.to_uppercase()),
                );
            }
            "dependencies" => {
                let mut cursor = value.walk();
                dependencies.extend(
                    value
                        .named_children(&mut cursor)
                        .map(|d| shown(text(d, source))),
                );
            }
            _ => {}
        }
    }
    let method = if METHODS.contains(&name) {
        name.to_uppercase()
    } else if name == "route" || name == "api_route" {
        if methods.is_empty() {
            // Flask's default; FastAPI's `api_route` has the same.
            "GET".to_owned()
        } else {
            methods.join(", ")
        }
    } else {
        return None;
    };
    Some((method, path, dependencies))
}

/// `app.get('/x', requireAuth, handler)` on an object named as an app or a router: its line, method, path, and the
/// arguments between the path and the handler.
fn script_route(node: Node, source: &str) -> Option<(usize, String, String, Vec<String>)> {
    if node.kind() != "call_expression" {
        return None;
    }
    let function = node.child_by_field_name("function")?;
    if function.kind() != "member_expression" {
        return None;
    }
    let method = text(function.child_by_field_name("property")?, source);
    if !METHODS.contains(&method) && method != "all" {
        return None;
    }
    let object = function.child_by_field_name("object")?;
    if object.kind() != "identifier" || !is_app_or_router(text(object, source)) {
        return None;
    }
    let arguments = node.child_by_field_name("arguments")?;
    let mut cursor = arguments.walk();
    let args: Vec<Node> = arguments
        .named_children(&mut cursor)
        .filter(|a| a.kind() != "comment")
        .collect();
    let path = literal(*args.first()?, source)?;
    if !path.starts_with('/') || args.len() < 2 {
        return None;
    }
    let guards = args[1..args.len() - 1]
        .iter()
        .map(|g| shown(text(*g, source)))
        .collect();
    Some((
        node.start_position().row + 1,
        method.to_uppercase(),
        path,
        guards,
    ))
}

/// Whether a name is one an Express-style app or router is given: `app`, `server`, `router`, `adminRouter`, `api`.
fn is_app_or_router(name: &str) -> bool {
    let lower = name.to_lowercase();
    matches!(lower.as_str(), "app" | "server" | "api" | "routes")
        || lower.ends_with("router")
        || lower.ends_with("app")
}

mod wider;

pub use wider::EVERY_ROUTE;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod wider_tests;
