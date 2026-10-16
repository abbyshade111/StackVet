//! More ways of declaring routes and their guards (backlog 0608): a check applied to every route at once, Django's
//! `urls.py` with its views' decorators and mixins, and Go's handlers. Like the rest of the table, where to look and
//! never evidence.

use super::{METHODS, RouteLine, literal, shown, text, walk};
use std::collections::BTreeMap;
use tree_sitter::{Node, Parser};

/// What the method column says for a check applied to every route at once.
pub const EVERY_ROUTE: &str = "EVERY ROUTE";

/// The words in a middleware's name that say it decides who may use the app. `app.use(express.json())` is not a check
/// of anyone; `app.use(requireAuth)` is meant to be one.
fn names_a_check(code: &str) -> bool {
    let lower = code.to_lowercase();
    [
        "auth",
        "login",
        "session",
        "guard",
        "permission",
        "role",
        "admin",
        "require",
        "ensure",
        "protect",
        "verify",
        "jwt",
        "passport",
        "signedin",
        "signed_in",
        "loggedin",
        "logged_in",
    ]
    .iter()
    .any(|w| lower.contains(w))
}

/// `app.use(requireAuth)`, `app.use('/admin', requireAdmin)`, `router.use(ensureSignedIn)`: a row for each middleware
/// whose name says it is a check, applied to every route under the path given, or every route.
pub(super) fn script_every_route(
    node: Node,
    source: &str,
) -> Option<(usize, String, String, Vec<String>)> {
    if node.kind() != "call_expression" {
        return None;
    }
    let function = node.child_by_field_name("function")?;
    if function.kind() != "member_expression"
        || text(function.child_by_field_name("property")?, source) != "use"
    {
        return None;
    }
    let object = function.child_by_field_name("object")?;
    if object.kind() != "identifier" || !super::is_app_or_router(text(object, source)) {
        return None;
    }
    let arguments = node.child_by_field_name("arguments")?;
    let mut cursor = arguments.walk();
    let args: Vec<Node> = arguments
        .named_children(&mut cursor)
        .filter(|a| a.kind() != "comment")
        .collect();
    let (path, rest) = match args.first().and_then(|a| literal(*a, source)) {
        Some(p) if p.starts_with('/') => (
            format!("{}*", p.trim_end_matches('/').to_owned() + "/"),
            &args[1..],
        ),
        _ => ("*".to_owned(), &args[..]),
    };
    let guards: Vec<String> = rest
        .iter()
        .map(|a| text(*a, source))
        .filter(|a| names_a_check(a) && !a.to_lowercase().ends_with("router"))
        .map(shown)
        .collect();
    (!guards.is_empty()).then(|| {
        (
            node.start_position().row + 1,
            EVERY_ROUTE.to_owned(),
            path,
            guards,
        )
    })
}

/// `@app.before_request` (Flask), and `@bp.before_app_request`: the function runs before every request.
pub(super) fn python_every_route(
    node: Node,
    source: &str,
) -> Option<(usize, String, String, Vec<String>)> {
    if node.kind() != "decorated_definition" {
        return None;
    }
    let definition = node.child_by_field_name("definition")?;
    let name = text(definition.child_by_field_name("name")?, source);
    let mut cursor = node.walk();
    let hook = node.children(&mut cursor).find(|d| {
        d.kind() == "decorator"
            && d.named_child(0).is_some_and(|e| {
                let e = text(e, source);
                e.ends_with(".before_request") || e.ends_with(".before_app_request")
            })
    })?;
    Some((
        hook.start_position().row + 1,
        EVERY_ROUTE.to_owned(),
        "*".to_owned(),
        vec![shown(name)],
    ))
}

/// Go: `http.HandleFunc("/x", h)`, `mux.Handle("/x", requireAuth(h))`, Go 1.22's `mux.HandleFunc("GET /x", h)`, and
/// router calls such as chi's `r.Get("/x", h)` and gin's `r.GET("/x", authRequired(), h)`; and `r.Use(mw)`, a check
/// on every route, when its name says it is one. With a handler wrapped in calls, the wrappers are what is written
/// on it; with several handlers, every one but the last.
pub(super) fn go_route(node: Node, source: &str) -> Option<(usize, String, String, Vec<String>)> {
    if node.kind() != "call_expression" {
        return None;
    }
    let function = node.child_by_field_name("function")?;
    if function.kind() != "selector_expression" {
        return None;
    }
    let name = text(function.child_by_field_name("field")?, source);
    let arguments = node.child_by_field_name("arguments")?;
    let mut cursor = arguments.walk();
    let args: Vec<Node> = arguments
        .named_children(&mut cursor)
        .filter(|a| a.kind() != "comment")
        .collect();
    let line = node.start_position().row + 1;
    if name == "Use" {
        let guards: Vec<String> = args
            .iter()
            .map(|a| text(*a, source))
            .filter(|a| names_a_check(a))
            .map(shown)
            .collect();
        return (!guards.is_empty())
            .then(|| (line, EVERY_ROUTE.to_owned(), "*".to_owned(), guards));
    }
    let lower = name.to_lowercase();
    let handle = matches!(name, "HandleFunc" | "Handle");
    if !handle && !METHODS.contains(&lower.as_str()) && name != "Any" {
        return None;
    }
    let first = *args.first()?;
    if !matches!(
        first.kind(),
        "interpreted_string_literal" | "raw_string_literal"
    ) || args.len() < 2
    {
        return None;
    }
    let pattern = text(first, source)
        .trim_matches(|c| c == '"' || c == '`')
        .to_owned();
    let (method, path) = match pattern.split_once(' ') {
        Some((m, p)) if handle && p.starts_with('/') => (m.to_uppercase(), p.to_owned()),
        _ if handle => ("ANY".to_owned(), pattern.clone()),
        _ => (
            if name == "Any" {
                "ANY".to_owned()
            } else {
                lower.to_uppercase()
            },
            pattern.clone(),
        ),
    };
    if !path.starts_with('/') {
        return None;
    }
    let mut guards: Vec<String> = args[1..args.len() - 1]
        .iter()
        .map(|a| shown(text(*a, source)))
        .collect();
    // The wrappers around the last handler: `requireAuth(adminOnly(h))` names both.
    let mut handler = *args.last()?;
    while handler.kind() == "call_expression" {
        let Some(callee) = handler.child_by_field_name("function") else {
            break;
        };
        let Some(inner) = handler
            .child_by_field_name("arguments")
            .and_then(|a| a.named_child(0))
        else {
            break;
        };
        // `http.HandlerFunc(h)` makes a function a handler and checks nothing.
        if !text(callee, source).ends_with("HandlerFunc") {
            guards.push(shown(text(callee, source)));
        }
        handler = inner;
    }
    Some((line, method, path, guards))
}

/// What is written on each Python function and class that could be a Django view, by its name: decorators other than
/// a route decorator, the mixins a class is built on, and a class's `permission_classes`.
pub(super) fn python_view_guards(source: &str) -> BTreeMap<String, Vec<String>> {
    let mut out = BTreeMap::new();
    let Some(tree) = parse("python", source) else {
        return out;
    };
    walk(tree.root_node(), &mut |node| {
        let (definition, decorators) = if node.kind() == "decorated_definition" {
            let Some(d) = node.child_by_field_name("definition") else {
                return;
            };
            let mut cursor = node.walk();
            let decorators: Vec<String> = node
                .children(&mut cursor)
                .filter(|c| c.kind() == "decorator")
                .filter_map(|c| c.named_child(0))
                .map(|e| shown(text(e, source)))
                .collect();
            (d, decorators)
        } else if matches!(node.kind(), "function_definition" | "class_definition")
            && node
                .parent()
                .is_none_or(|p| p.kind() != "decorated_definition")
        {
            (node, Vec::new())
        } else {
            return;
        };
        let Some(name) = definition.child_by_field_name("name") else {
            return;
        };
        let mut guards = decorators;
        if definition.kind() == "class_definition" {
            if let Some(bases) = definition.child_by_field_name("superclasses") {
                let mut cursor = bases.walk();
                guards.extend(
                    bases
                        .named_children(&mut cursor)
                        .map(|b| text(b, source))
                        .filter(|b| b.ends_with("Mixin"))
                        .map(shown),
                );
            }
            if let Some(body) = definition.child_by_field_name("body") {
                let mut cursor = body.walk();
                for statement in body.named_children(&mut cursor) {
                    let line = text(statement, source);
                    if line.starts_with("permission_classes")
                        || line.starts_with("authentication_classes")
                    {
                        guards.push(shown(line));
                    }
                }
            }
        }
        out.entry(text(name, source).to_owned()).or_insert(guards);
    });
    out
}

/// Django's `path('notes/', views.notes)` and `re_path(...)` in a `urls.py`, each with what is written on its view in
/// `views` (by the view's name) and any wrapper around it in `urls.py` itself (`login_required(views.notes)`).
pub(super) fn django_routes(
    file: &str,
    source: &str,
    views: &BTreeMap<String, Vec<String>>,
) -> Vec<RouteLine> {
    let mut out = Vec::new();
    let Some(tree) = parse("python", source) else {
        return out;
    };
    walk(tree.root_node(), &mut |node| {
        if node.kind() != "call" {
            return;
        }
        let Some(function) = node.child_by_field_name("function") else {
            return;
        };
        let callee = text(function, source);
        if !matches!(callee, "path" | "re_path" | "url") {
            return;
        }
        let Some(arguments) = node.child_by_field_name("arguments") else {
            return;
        };
        let mut cursor = arguments.walk();
        let args: Vec<Node> = arguments.named_children(&mut cursor).collect();
        let (Some(route), Some(view)) =
            (args.first().and_then(|a| literal(*a, source)), args.get(1))
        else {
            return;
        };
        let mut guards = Vec::new();
        let mut view = *view;
        // `login_required(views.notes)`: the wrapper is what is written on it.
        while view.kind() == "call" {
            let Some(inner_fn) = view.child_by_field_name("function") else {
                break;
            };
            let name = text(inner_fn, source);
            if name.ends_with("as_view") {
                view = inner_fn.child_by_field_name("object").unwrap_or(inner_fn);
                break;
            }
            if name == "include" {
                return;
            }
            let Some(inner) = view
                .child_by_field_name("arguments")
                .and_then(|a| a.named_child(0))
            else {
                break;
            };
            guards.push(shown(name));
            view = inner;
        }
        let view_name = text(view, source)
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_owned();
        guards.extend(views.get(&view_name).cloned().unwrap_or_default());
        let path = format!("/{}", route.trim_start_matches('^').trim_start_matches('/'));
        out.push(RouteLine {
            file: file.to_owned(),
            line: node.start_position().row + 1,
            method: "ANY".to_owned(),
            path,
            guards,
        });
    });
    out
}

fn parse(key: &str, source: &str) -> Option<tree_sitter::Tree> {
    let grammar = crate::ast::grammar(key)?;
    let mut parser = Parser::new();
    parser.set_language(&grammar).ok()?;
    parser.parse(source, None)
}
