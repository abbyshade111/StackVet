//! Routes and their guards in four more frameworks (backlog 0608, second part): Spring's mappings with `@PreAuthorize`,
//! ASP.NET's attributes and minimal API, Laravel's `Route::` calls with their middleware, and Rails' `routes.rb` with
//! its controllers' `before_action`. Like the rest of the table, where to look and never evidence.

use super::{METHODS, RouteLine, shown, text, walk};
use std::collections::BTreeMap;
use tree_sitter::{Node, Parser};

type Found = (usize, String, String, Vec<String>);

/// The text of a string node's content, whatever the grammar calls its parts; `None` when it is not a plain string.
fn string_value(node: Node, source: &str) -> Option<String> {
    if !matches!(
        node.kind(),
        "string" | "string_literal" | "encapsed_string" | "interpreted_string_literal"
    ) {
        return None;
    }
    let raw = text(node, source);
    if raw.contains("${")
        || raw.contains("#{")
        || raw.contains('{') && node.kind() == "encapsed_string"
    {
        return None;
    }
    Some(
        raw.trim_matches(|c| c == '"' || c == '\'' || c == '@')
            .to_owned(),
    )
}

/// Joins a prefix and a path as a route: `/api` and `/notes` make `/api/notes`.
fn joined(prefix: &str, path: &str) -> String {
    let both = format!(
        "{}/{}",
        prefix.trim_end_matches('/'),
        path.trim_start_matches('/')
    );
    if both.starts_with('/') {
        both
    } else {
        format!("/{both}")
    }
}

// ---- Spring (Java)

const SPRING_GUARDS: &[&str] = &[
    "PreAuthorize",
    "PostAuthorize",
    "Secured",
    "RolesAllowed",
    "PermitAll",
    "DenyAll",
];

/// The annotations in a `modifiers` node, each as its name and the node.
fn annotations<'a>(modifiers: Option<Node<'a>>, source: &str) -> Vec<(String, Node<'a>)> {
    let Some(modifiers) = modifiers else {
        return Vec::new();
    };
    let mut cursor = modifiers.walk();
    modifiers
        .named_children(&mut cursor)
        .filter(|a| matches!(a.kind(), "annotation" | "marker_annotation"))
        .filter_map(|a| {
            a.child_by_field_name("name")
                .map(|n| (text(n, source).to_owned(), a))
        })
        .collect()
}

fn modifiers_of(node: Node) -> Option<Node> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .find(|c| c.kind() == "modifiers")
}

/// A mapping annotation's path: its first string, or its `value =` or `path =`.
fn mapping_path(annotation: Node, source: &str) -> Option<String> {
    let arguments = annotation.child_by_field_name("arguments")?;
    let mut cursor = arguments.walk();
    for argument in arguments.named_children(&mut cursor) {
        if let Some(path) = string_value(argument, source) {
            return Some(path);
        }
        if argument.kind() == "element_value_pair"
            && argument
                .child_by_field_name("key")
                .is_some_and(|k| matches!(text(k, source), "value" | "path"))
        {
            return argument
                .child_by_field_name("value")
                .and_then(|v| string_value(v, source));
        }
    }
    None
}

/// `@RequestMapping(method = RequestMethod.POST)`: the method it names, `ANY` when none.
fn request_mapping_method(annotation: Node, source: &str) -> String {
    let all = text(annotation, source);
    METHODS
        .iter()
        .find(|m| all.contains(&format!("RequestMethod.{}", m.to_uppercase())))
        .map_or("ANY".to_owned(), |m| m.to_uppercase())
}

pub(super) fn spring_route(node: Node, source: &str) -> Option<Found> {
    if node.kind() != "method_declaration" {
        return None;
    }
    let mut route = None;
    let mut guards = Vec::new();
    for (name, annotation) in annotations(modifiers_of(node), source) {
        let method = match name.as_str() {
            "GetMapping" => "GET".to_owned(),
            "PostMapping" => "POST".to_owned(),
            "PutMapping" => "PUT".to_owned(),
            "DeleteMapping" => "DELETE".to_owned(),
            "PatchMapping" => "PATCH".to_owned(),
            "RequestMapping" => request_mapping_method(annotation, source),
            n if SPRING_GUARDS.contains(&n) => {
                guards.push(shown(text(annotation, source)));
                continue;
            }
            _ => continue,
        };
        route = Some((method, mapping_path(annotation, source).unwrap_or_default()));
    }
    let (method, path) = route?;
    // The class's own prefix and guards.
    let mut prefix = String::new();
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "class_declaration" {
            for (name, annotation) in annotations(modifiers_of(p), source) {
                if name == "RequestMapping" {
                    prefix = mapping_path(annotation, source).unwrap_or_default();
                } else if SPRING_GUARDS.contains(&name.as_str()) {
                    guards.push(format!("on the class: {}", shown(text(annotation, source))));
                }
            }
            break;
        }
        parent = p.parent();
    }
    Some((
        node.start_position().row + 1,
        method,
        joined(&prefix, &path),
        guards,
    ))
}

// ---- ASP.NET (C#)

const ASPNET_VERBS: &[(&str, &str)] = &[
    ("HttpGet", "GET"),
    ("HttpPost", "POST"),
    ("HttpPut", "PUT"),
    ("HttpDelete", "DELETE"),
    ("HttpPatch", "PATCH"),
];

/// The attributes on a declaration, each as its name and the node.
fn attributes<'a>(node: Node<'a>, source: &str) -> Vec<(String, Node<'a>)> {
    let mut out = Vec::new();
    let mut cursor = node.walk();
    for list in node
        .named_children(&mut cursor)
        .filter(|c| c.kind() == "attribute_list")
    {
        let mut inner = list.walk();
        for attribute in list
            .named_children(&mut inner)
            .filter(|a| a.kind() == "attribute")
        {
            if let Some(name) = attribute.child_by_field_name("name") {
                out.push((text(name, source).to_owned(), attribute));
            }
        }
    }
    out
}

fn attribute_path(attribute: Node, source: &str) -> Option<String> {
    let mut found = None;
    walk(attribute, &mut |n| {
        if found.is_none() && n.kind() == "string_literal" {
            found = string_value(n, source);
        }
    });
    found
}

pub(super) fn aspnet_route(node: Node, source: &str) -> Option<Found> {
    match node.kind() {
        "method_declaration" => aspnet_controller_route(node, source),
        "invocation_expression" => aspnet_minimal_route(node, source),
        _ => None,
    }
}

fn aspnet_controller_route(node: Node, source: &str) -> Option<Found> {
    let mut route: Option<(String, String)> = None;
    let mut guards = Vec::new();
    for (name, attribute) in attributes(node, source) {
        if let Some((_, verb)) = ASPNET_VERBS.iter().find(|(a, _)| *a == name) {
            let path = attribute_path(attribute, source).unwrap_or_default();
            route = Some(((*verb).to_owned(), path));
        } else if name == "Route" && route.is_none() {
            route = Some((
                "ANY".to_owned(),
                attribute_path(attribute, source).unwrap_or_default(),
            ));
        } else if matches!(name.as_str(), "Authorize" | "AllowAnonymous") {
            guards.push(shown(text(attribute, source)));
        }
    }
    let (method, path) = route?;
    let mut prefix = String::new();
    let mut parent = node.parent();
    while let Some(p) = parent {
        if p.kind() == "class_declaration" {
            for (name, attribute) in attributes(p, source) {
                if name == "Route" {
                    prefix = attribute_path(attribute, source).unwrap_or_default();
                } else if matches!(name.as_str(), "Authorize" | "AllowAnonymous") {
                    guards.push(format!("on the class: {}", shown(text(attribute, source))));
                }
            }
            break;
        }
        parent = p.parent();
    }
    Some((
        node.start_position().row + 1,
        method,
        joined(&prefix, &path),
        guards,
    ))
}

/// `app.MapGet("/x", ...)`, with `.RequireAuthorization(...)` or `.AllowAnonymous()` chained on it.
fn aspnet_minimal_route(node: Node, source: &str) -> Option<Found> {
    let function = node.child_by_field_name("function")?;
    if function.kind() != "member_access_expression" {
        return None;
    }
    let name = text(function.child_by_field_name("name")?, source);
    let method = match name {
        "MapGet" => "GET",
        "MapPost" => "POST",
        "MapPut" => "PUT",
        "MapDelete" => "DELETE",
        "MapPatch" => "PATCH",
        "MapMethods" | "Map" => "ANY",
        _ => return None,
    };
    let arguments = node.child_by_field_name("arguments")?;
    let first = arguments.named_child(0)?;
    let path = attribute_path(first, source)?;
    if !path.starts_with('/') {
        return None;
    }
    let mut guards = Vec::new();
    // The calls chained after it: `.RequireAuthorization("admin")` is `(member_access (invocation ...) name)`.
    let mut outer = node;
    while let Some(access) = outer
        .parent()
        .filter(|p| p.kind() == "member_access_expression")
    {
        let Some(call) = access
            .parent()
            .filter(|p| p.kind() == "invocation_expression")
        else {
            break;
        };
        let called = access
            .child_by_field_name("name")
            .map_or("", |n| text(n, source));
        if matches!(called, "RequireAuthorization" | "AllowAnonymous") {
            guards.push(shown(
                &text(call, source)[access
                    .child_by_field_name("name")
                    .map_or(0, |n| n.start_byte() - call.start_byte())..],
            ));
        }
        outer = call;
    }
    Some((
        node.start_position().row + 1,
        method.to_owned(),
        path,
        guards,
    ))
}

// ---- Laravel (PHP)

/// `Route::get('/x', ...)`, with `->middleware(...)` chained on it and any `Route::middleware(...)->group(...)` it is
/// written inside.
pub(super) fn laravel_route(node: Node, source: &str) -> Option<Found> {
    if node.kind() != "scoped_call_expression" {
        return None;
    }
    if text(node.child_by_field_name("scope")?, source) != "Route" {
        return None;
    }
    let name = text(node.child_by_field_name("name")?, source).to_lowercase();
    if !METHODS.contains(&name.as_str()) && name != "any" && name != "match" {
        return None;
    }
    let arguments = node.child_by_field_name("arguments")?;
    let mut cursor = arguments.walk();
    let args: Vec<Node> = arguments.named_children(&mut cursor).collect();
    let path_arg = if name == "match" {
        args.get(1)
    } else {
        args.first()
    };
    let path = path_arg
        .and_then(|a| a.named_child(0))
        .and_then(|s| string_value(s, source))?;
    let method = if name == "match" {
        "ANY".to_owned()
    } else {
        name.to_uppercase()
    };
    let mut guards = Vec::new();
    // `->middleware('auth')` chained on this route.
    let mut outer = node;
    while let Some(call) = outer
        .parent()
        .filter(|p| p.kind() == "member_call_expression")
    {
        if call
            .child_by_field_name("name")
            .is_some_and(|n| text(n, source) == "middleware")
            && let Some(a) = call.child_by_field_name("arguments")
        {
            guards.push(format!("middleware{}", shown(text(a, source))));
        }
        outer = call;
    }
    // Every `->group(function () { ... })` around it, and the middleware that group is built on.
    let mut parent = outer.parent();
    while let Some(p) = parent {
        if p.kind() == "member_call_expression"
            && p.child_by_field_name("name")
                .is_some_and(|n| text(n, source) == "group")
        {
            walk(p.child_by_field_name("object").unwrap_or(p), &mut |n| {
                let is_middleware = matches!(
                    n.kind(),
                    "member_call_expression" | "scoped_call_expression"
                ) && n
                    .child_by_field_name("name")
                    .is_some_and(|m| text(m, source) == "middleware");
                if is_middleware && let Some(a) = n.child_by_field_name("arguments") {
                    guards.push(format!("in a group: middleware{}", shown(text(a, source))));
                }
            });
        }
        parent = p.parent();
    }
    let path = if path.starts_with('/') {
        path
    } else {
        format!("/{path}")
    };
    Some((node.start_position().row + 1, method, path, guards))
}

// ---- Rails (Ruby)

/// What each controller has written on it, by the name `routes.rb` uses for it (`notes` for `NotesController`):
/// `before_action` and `skip_before_action` lines, and those of `ApplicationController` for those built on it.
pub(super) fn rails_controller_guards(files: &[(String, String)]) -> BTreeMap<String, Vec<String>> {
    let mut own: BTreeMap<String, (Option<String>, Vec<String>)> = BTreeMap::new();
    for (_, source) in files {
        let Some(tree) = parse("ruby", source) else {
            continue;
        };
        walk(tree.root_node(), &mut |node| {
            if node.kind() != "class" {
                return;
            }
            let Some(name) = node.child_by_field_name("name") else {
                return;
            };
            let superclass = node
                .child_by_field_name("superclass")
                .and_then(|s| s.named_child(0))
                .map(|s| text(s, source).to_owned());
            let mut lines = Vec::new();
            if let Some(body) = node.child_by_field_name("body") {
                let mut cursor = body.walk();
                for statement in body.named_children(&mut cursor) {
                    let line = text(statement, source);
                    if statement.kind() == "call"
                        && [
                            "before_action",
                            "skip_before_action",
                            "before_filter",
                            "http_basic_authenticate_with",
                        ]
                        .iter()
                        .any(|w| line.starts_with(w))
                    {
                        lines.push(shown(line));
                    }
                }
            }
            own.insert(text(name, source).to_owned(), (superclass, lines));
        });
    }
    let application = own
        .get("ApplicationController")
        .map(|(_, l)| l.clone())
        .unwrap_or_default();
    own.iter()
        .filter_map(|(class, (superclass, lines))| {
            let key = class.strip_suffix("Controller")?;
            let mut guards = Vec::new();
            if superclass.as_deref() == Some("ApplicationController") {
                guards.extend(
                    application
                        .iter()
                        .map(|l| format!("in ApplicationController: {l}")),
                );
            }
            guards.extend(lines.iter().cloned());
            Some((snake(key), guards))
        })
        .collect()
}

/// `NoteItems` as Rails names its routes: `note_items`.
fn snake(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

/// The routes in a `routes.rb`: `get '/x', to: 'notes#index'`, `post 'login' => 'sessions#create'`, `resources :notes`,
/// and `root 'home#index'`, each with what is written on its controller.
pub(super) fn rails_routes(
    file: &str,
    source: &str,
    controllers: &BTreeMap<String, Vec<String>>,
) -> Vec<RouteLine> {
    let mut out = Vec::new();
    let Some(tree) = parse("ruby", source) else {
        return out;
    };
    walk(tree.root_node(), &mut |node| {
        if node.kind() != "call" || node.child_by_field_name("receiver").is_some() {
            return;
        }
        let Some(method) = node.child_by_field_name("method").map(|m| text(m, source)) else {
            return;
        };
        let Some(arguments) = node.child_by_field_name("arguments") else {
            return;
        };
        let mut cursor = arguments.walk();
        let args: Vec<Node> = arguments.named_children(&mut cursor).collect();
        let (verb, path, controller) = match method {
            "resources" | "resource" => {
                let Some(name) = args
                    .first()
                    .map(|a| text(*a, source).trim_start_matches(':').to_owned())
                else {
                    return;
                };
                ("RESOURCES".to_owned(), format!("/{name}"), name)
            }
            "root" => {
                let target = args
                    .first()
                    .and_then(|a| string_value(*a, source))
                    .unwrap_or_default();
                (
                    "GET".to_owned(),
                    "/".to_owned(),
                    target.split('#').next().unwrap_or("").to_owned(),
                )
            }
            m if METHODS.contains(&m) || m == "match" => {
                let (mut path, mut target) = (None, None);
                for a in &args {
                    if let Some(s) = string_value(*a, source) {
                        path.get_or_insert(s);
                    } else if a.kind() == "pair" {
                        let key = a.child_by_field_name("key").map_or("", |k| text(k, source));
                        let value = a
                            .child_by_field_name("value")
                            .and_then(|v| string_value(v, source));
                        if key.trim_end_matches(':') == "to" {
                            target = value;
                        } else if let Some(k) = a
                            .child_by_field_name("key")
                            .and_then(|k| string_value(k, source))
                        {
                            path.get_or_insert(k);
                            target = value;
                        }
                    }
                }
                let Some(path) = path else { return };
                let controller = target
                    .as_deref()
                    .and_then(|t| t.split('#').next())
                    .unwrap_or("")
                    .to_owned();
                let verb = if m == "match" {
                    "ANY".to_owned()
                } else {
                    m.to_uppercase()
                };
                (
                    verb,
                    if path.starts_with('/') {
                        path
                    } else {
                        format!("/{path}")
                    },
                    controller,
                )
            }
            _ => return,
        };
        out.push(RouteLine {
            file: file.to_owned(),
            line: node.start_position().row + 1,
            method: verb,
            path,
            guards: controllers.get(&controller).cloned().unwrap_or_default(),
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
