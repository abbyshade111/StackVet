//! YAML, read into a [`Value`] whose scalars keep their lines, with the `tree-sitter-yaml` grammar: for the workflow
//! checks (`crate::workflows`) and the outbound rules a folder's infrastructure files set (`crate::outbound_rules`,
//! backlog 0496). An anchor, an alias, or a tag is refused rather than followed, since following either is where a
//! reading goes wrong quietly. Moved out of `workflows.rs` on 22 October 2026, word for word, but for the documents
//! a Kubernetes file holds several of (`parse_documents`).

use tree_sitter::{Node, Parser};

/// A YAML value, as far as `sv`'s readers need one: every scalar keeps the line it is on.
#[derive(Debug, Clone)]
pub(crate) enum Value {
    Scalar(String, usize),
    Seq(Vec<Value>),
    Map(Vec<(String, Value)>),
    Null,
}

impl Value {
    pub(crate) fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Map(pairs) => pairs.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    pub(crate) fn text(&self) -> Option<&str> {
        match self {
            Value::Scalar(s, _) => Some(s),
            _ => None,
        }
    }

    /// The line of the first scalar in this value, for a finding to point at.
    pub(crate) fn line(&self) -> Option<usize> {
        match self {
            Value::Scalar(_, line) => Some(*line),
            Value::Seq(items) => items.iter().find_map(Value::line),
            Value::Map(pairs) => pairs.iter().find_map(|(_, v)| v.line()),
            Value::Null => None,
        }
    }

    /// Every scalar inside this value with its line, keys left out.
    pub(crate) fn scalars_at<'a>(&'a self, out: &mut Vec<(&'a str, usize)>) {
        match self {
            Value::Scalar(s, line) => out.push((s, *line)),
            Value::Seq(items) => items.iter().for_each(|v| v.scalars_at(out)),
            Value::Map(pairs) => pairs.iter().for_each(|(_, v)| v.scalars_at(out)),
            Value::Null => {}
        }
    }

    /// Every scalar inside this value, keys included.
    pub(crate) fn scalars<'a>(&'a self, keys: bool, out: &mut Vec<&'a str>) {
        match self {
            Value::Scalar(s, _) => out.push(s),
            Value::Seq(items) => items.iter().for_each(|v| v.scalars(keys, out)),
            Value::Map(pairs) => {
                for (k, v) in pairs {
                    if keys {
                        out.push(k);
                    }
                    v.scalars(keys, out);
                }
            }
            Value::Null => {}
        }
    }
}

/// Reads a file of one YAML document into a [`Value`], or says why it could not.
pub(crate) fn parse(source: &str) -> Result<Value, String> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_yaml::LANGUAGE.into())
        .map_err(|_| "the YAML grammar could not be loaded".to_owned())?;
    let tree = parser
        .parse(source, None)
        .ok_or_else(|| "the file could not be parsed".to_owned())?;
    let root = tree.root_node();
    if root.has_error() {
        return Err("the file is not YAML the grammar can read".to_owned());
    }
    let mut cursor = root.walk();
    let documents: Vec<Node> = root
        .named_children(&mut cursor)
        .filter(|n| n.kind() == "document")
        .collect();
    match documents.as_slice() {
        [] => Ok(Value::Null),
        [one] => {
            let mut cursor = one.walk();
            let content: Vec<Node> = one
                .named_children(&mut cursor)
                .filter(|n| n.kind() != "comment")
                .collect();
            match content.as_slice() {
                [] => Ok(Value::Null),
                [node] => value(*node, source),
                _ => Err("the file holds something besides one document".to_owned()),
            }
        }
        _ => Err("the file holds more than one YAML document".to_owned()),
    }
}

fn value(node: Node, source: &str) -> Result<Value, String> {
    let text = |n: Node| {
        n.utf8_text(source.as_bytes())
            .unwrap_or_default()
            .to_owned()
    };
    let line = node.start_position().row + 1;
    match node.kind() {
        "block_node" | "flow_node" => {
            let mut cursor = node.walk();
            let children: Vec<Node> = node
                .named_children(&mut cursor)
                .filter(|n| n.kind() != "comment")
                .collect();
            if children
                .iter()
                .any(|n| matches!(n.kind(), "anchor" | "tag" | "alias"))
            {
                // An alias makes one value stand for another written elsewhere, and a tag can change
                // what a value means. Following either is where a reading goes wrong quietly.
                return Err("the file uses a YAML anchor, alias, or tag".to_owned());
            }
            match children.as_slice() {
                [] => Ok(Value::Null),
                [inner] => value(*inner, source),
                _ => Err(format!("a value on line {line} could not be read")),
            }
        }
        "block_mapping" | "flow_mapping" => {
            let mut pairs = Vec::new();
            let mut cursor = node.walk();
            for pair in node.named_children(&mut cursor) {
                match pair.kind() {
                    "comment" => {}
                    "block_mapping_pair" | "flow_pair" => {
                        let key = match pair.child_by_field_name("key") {
                            Some(k) => match value(k, source)? {
                                Value::Scalar(s, _) => s,
                                _ => return Err(format!("a key on line {line} is not plain text")),
                            },
                            None => String::new(),
                        };
                        let val = match pair.child_by_field_name("value") {
                            Some(v) => value(v, source)?,
                            None => Value::Null,
                        };
                        pairs.push((key, val));
                    }
                    // A flow mapping entry with a key and no value: `{ push }`.
                    "flow_node" => {
                        if let Value::Scalar(s, _) = value(pair, source)? {
                            pairs.push((s, Value::Null));
                        }
                    }
                    other => return Err(format!("unexpected `{other}` on line {line}")),
                }
            }
            Ok(Value::Map(pairs))
        }
        "block_sequence" | "flow_sequence" => {
            let mut items = Vec::new();
            let mut cursor = node.walk();
            for item in node.named_children(&mut cursor) {
                match item.kind() {
                    "comment" => {}
                    "block_sequence_item" => {
                        let mut inner_cursor = item.walk();
                        let inner: Vec<Node> = item
                            .named_children(&mut inner_cursor)
                            .filter(|n| n.kind() != "comment")
                            .collect();
                        items.push(match inner.as_slice() {
                            [] => Value::Null,
                            [one] => value(*one, source)?,
                            _ => {
                                return Err(format!(
                                    "a list item on line {line} could not be read"
                                ));
                            }
                        });
                    }
                    _ => items.push(value(item, source)?),
                }
            }
            Ok(Value::Seq(items))
        }
        "plain_scalar" => Ok(Value::Scalar(text(node).trim().to_owned(), line)),
        "single_quote_scalar" => {
            let raw = text(node);
            let inner = raw
                .strip_prefix('\'')
                .and_then(|s| s.strip_suffix('\''))
                .unwrap_or(&raw);
            Ok(Value::Scalar(inner.replace("''", "'"), line))
        }
        "double_quote_scalar" => {
            let raw = text(node);
            let inner = raw
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .unwrap_or(&raw);
            Ok(Value::Scalar(
                inner.replace("\\\"", "\"").replace("\\\\", "\\"),
                line,
            ))
        }
        // `run: |` and `run: >`. Only ever searched for text, so the indicator line is harmless.
        "block_scalar" => Ok(Value::Scalar(text(node), line)),
        other => Err(format!("unexpected `{other}` on line {line}")),
    }
}

/// Reads a file of one or more YAML documents, separated by `---` as Kubernetes manifests often are, into a [`Value`]
/// each, or says why it could not. A document with nothing in it is left out.
pub(crate) fn parse_documents(source: &str) -> Result<Vec<Value>, String> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_yaml::LANGUAGE.into())
        .map_err(|_| "the YAML grammar could not be loaded".to_owned())?;
    let tree = parser
        .parse(source, None)
        .ok_or_else(|| "the file could not be parsed".to_owned())?;
    let root = tree.root_node();
    if root.has_error() {
        return Err("the file is not YAML the grammar can read".to_owned());
    }
    let mut cursor = root.walk();
    let mut out = Vec::new();
    for document in root
        .named_children(&mut cursor)
        .filter(|n| n.kind() == "document")
    {
        let mut inner = document.walk();
        let content: Vec<Node> = document
            .named_children(&mut inner)
            .filter(|n| n.kind() != "comment")
            .collect();
        match content.as_slice() {
            [] => {}
            [node] => out.push(value(*node, source)?),
            _ => return Err("a document holds something besides one value".to_owned()),
        }
    }
    Ok(out)
}
