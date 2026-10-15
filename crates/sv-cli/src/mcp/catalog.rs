//! What the server offers, in the words the AI coding tool reads: the tool list and each tool's
//! output shape, the design-time prompts, the starter specification, and a requirement's text.

use super::*;

/// The tools, each with the shape of its structured result where it has one.
pub(super) fn tools() -> Value {
    let mut list = tool_list();
    for tool in list.as_array_mut().into_iter().flatten() {
        let name = tool["name"].as_str().unwrap_or_default().to_owned();
        // A tool that reads an app can quote it, in its result or in what went wrong (deep review R9).
        if tool["inputSchema"]["properties"].get("path").is_some() {
            let description = tool["description"].as_str().unwrap_or_default();
            tool["description"] = json!(format!("{description} {}", sv_report::fence::ABOUT));
        }
        if let Some(schema) = output_schema(&name) {
            tool["outputSchema"] = schema;
        }
    }
    list
}

/// The shape of each tool's `structuredContent`, so a client can rely on it (2025-06-18 protocol).
///
/// Every field is named, the ones always present are required, and no other field is allowed: a
/// field added to a result without being added here fails `every_structured_result_has_the_shape_its_tool_declares`,
/// so the declaration cannot fall behind what is sent. A tool that answers only in text declares none.
pub(super) fn output_schema(tool: &str) -> Option<Value> {
    let string = json!({ "type": "string" });
    let count = json!({ "type": "integer", "minimum": 0 });
    let strings = json!({ "type": "array", "items": string });
    let object = |properties: Value, required: &[&str]| json!({ "type": "object", "properties": properties, "required": required, "additionalProperties": false });
    // A finding named in a line, as a check gave it before (backlog 0336).
    let shown = object(
        json!({ "fingerprint": string, "ruleId": string, "file": string, "line": count }),
        &["fingerprint", "ruleId", "file", "line"],
    );
    // One gone since, and whether its line still reads as it did (backlog 0401).
    let gone = object(
        json!({
            "fingerprint": string, "ruleId": string, "file": string, "line": count,
            "stillReads": object(
                json!({
                    "line": count,
                    "how": { "type": "string", "enum": ["unchanged", "look-away"] },
                    "scanner": string,
                }),
                &["line", "how"],
            ),
        }),
        &["fingerprint", "ruleId", "file", "line"],
    );
    let finding = object(
        json!({
            "rule_id": string, "title": string,
            "severity": { "type": "string", "enum": ["critical", "high", "medium", "low", "info"] },
            "confidence": { "type": "string", "enum": ["high", "medium", "low"] },
            "location": object(json!({ "file": string, "line": count }), &["file", "line"]),
            "secret": {
                "type": ["object", "null"],
                "properties": { "redacted": string, "length": count },
                "required": ["redacted", "length"],
                "additionalProperties": false
            },
            "requirement_ids": strings, "cwe": strings,
            "description": string, "impact": string, "fix": string,
            "also_reported_by": strings, "fingerprint": string, "earlier_fingerprints": strings,
            "marked_test_code": { "type": "boolean" },
            "bundled_library": string,
            // Why the report lists it apart (ADR-023, Later, 6 October 2026).
            "outranked": object(
                json!({
                    "why": { "type": "string", "enum": ["not-held-to", "checked-while-running"] },
                    "check": string,
                }),
                &["why"],
            ),
            // The other problems on the same line, each a finding of this same shape.
            "also_on_this_line": { "type": "array", "items": { "type": "object" } },
        }),
        &[
            "rule_id",
            "title",
            "severity",
            "confidence",
            "location",
            "secret",
            "requirement_ids",
            "cwe",
            "description",
            "impact",
            "fix",
        ],
    );
    // Which pages of a long answer a part holds, and every section there is (`crate::parts`). Sent only when the
    // answer comes in parts, so it is not required.
    let page = object(
        json!({ "section": string, "page": count, "pages": count }),
        &["section", "page", "pages"],
    );
    let part = object(
        json!({
            "shown": { "type": "array", "items": page },
            "sections": { "type": "array", "items": object(
                json!({ "section": string, "title": string, "pages": count, "fields": strings }),
                &["section", "title", "pages", "fields"],
            ) },
            "howToAsk": string,
        }),
        &["shown", "sections", "howToAsk"],
    );
    let counts = [
        "applicable",
        "needs_attention",
        "checked",
        "checked_in_part",
        "app_tested",
        "documented",
        "attested",
        "stated",
        "by_hand",
        "not_verified",
        "not_applicable",
        "not_assessed",
        "out_of_level",
        "ai_process",
    ];
    // One question only the person can answer, as the check's `questions` lists it.
    let question = object(
        json!({
            "id": string, "title": string, "how": string,
            // The question in plain words, asked first; `how` is then the standard's words (backlog 0427).
            "plain": { "type": ["string", "null"] },
            "where_to_look": { "type": ["string", "null"] },
            "route": { "type": "string", "enum": ["write-it-down", "answer-in-the-manifest", "go-and-look"] },
            "where_means": { "type": ["string", "null"] },
            // Where in the code the question is about, file and line only (backlog 0487): where to look, not evidence.
            "rests_on": {
                "type": ["object", "null"],
                "properties": {
                    "finds": string,
                    "places": { "type": "array", "items": object(json!({ "file": string, "line": count }), &["file", "line"]) },
                    "more": count,
                },
                "required": ["finds", "places", "more"],
                "additionalProperties": false
            },
        }),
        &[
            "id",
            "title",
            "how",
            "plain",
            "where_to_look",
            "route",
            "where_means",
            "rests_on",
        ],
    );
    let schema = match tool {
        "stackvet_check" => object(
            json!({
                "app": string,
                "targetLevel": count,
                "counts": object(
                    Value::Object(counts.iter().map(|c| ((*c).to_owned(), count.clone())).collect()),
                    &counts,
                ),
                "notExamined": { "type": "array", "items": object(
                    json!({
                        "what": string,
                        "why": string,
                        "reason": { "type": "string", "enum": [
                            "not-asked", "not-installed", "could-not-read", "no-reader", "stopped",
                            "person-only", "planned", "partial", "left-out", "outdated",
                        ] },
                        "requirements": strings,
                    }),
                    &["what", "why", "reason"],
                ) },
                "findings": { "type": "array", "items": finding },
                "needsAttention": strings,
                "claims": { "type": "array", "items": object(
                    json!({
                        "name": string,
                        "claimed": { "type": ["boolean", "null"] },
                        "found_in_code": { "type": ["boolean", "null"] },
                        "state": string,
                        "note": string,
                    }),
                    &["name", "claimed", "found_in_code", "state", "note"],
                ) },
                "undecided": { "type": "array", "items": object(
                    json!({ "id": string, "description": string, "chapter": string, "blocked_on": strings }),
                    &["id", "description", "chapter", "blocked_on"],
                ) },
                "questions": { "type": "array", "items": question.clone() },
                "part": part.clone(),
                // Asked for with no section, the findings given before in this session and still
                // found, and those gone since, named a line each (backlog 0336).
                "unchanged": { "type": "array", "items": shown.clone() },
                "gone": { "type": "array", "items": gone },
                "since": string,
            }),
            // The lists are left out of a part that holds none of them (`crate::parts`).
            &["app", "targetLevel", "counts"],
        ),
        // Folded into `stackvet_check` on 9 October 2026, and still answering by this name, unlisted.
        "stackvet_questions" => object(
            json!({ "questions": { "type": "array", "items": question } }),
            &["questions"],
        ),
        "stackvet_guidance" => object(
            json!({
                "rules": { "type": "array", "items": object(
                    json!({ "id": string, "topic": string, "rule": string, "cites": strings }),
                    &["id", "topic", "rule", "cites"],
                ) },
                "prompts": { "type": "array", "items": object(
                    json!({ "id": string, "title": string, "text": string }),
                    &["id", "title", "text"],
                ) },
                "leftOut": count,
                "filteredBySecurevibeToml": { "type": "boolean" },
                "attribution": object(
                    json!({ "title": string, "authors": string, "url": string, "license": string, "licenseUrl": string, "changes": string }),
                    &["title", "authors", "url", "license", "licenseUrl", "changes"],
                ),
            }),
            &[
                "rules",
                "prompts",
                "leftOut",
                "filteredBySecurevibeToml",
                "attribution",
            ],
        ),
        "stackvet_plan" => {
            let item = |fields: &[&str]| {
                let properties: serde_json::Map<String, Value> = fields
                    .iter()
                    .map(|f| {
                        let kind = match *f {
                            "level" => count.clone(),
                            "given" => json!({ "type": "boolean" }),
                            _ => string.clone(),
                        };
                        ((*f).to_owned(), kind)
                    })
                    .collect();
                json!({ "type": "array", "items": object(Value::Object(properties), fields) })
            };
            object(
                json!({
                    "app": string, "level": count,
                    "requirements": item(&["id", "level", "chapter", "description"]),
                    "decisions": item(&["id", "title"]),
                    "prompts": item(&["id", "title", "status"]),
                    "tests": item(&["id", "level", "description"]),
                    "run": item(&["table", "key", "why", "given"]),
                    "threats": item(&["id", "description", "status"]),
                    "creditsNothing": { "type": "boolean" },
                    "part": part.clone(),
                }),
                // The lists are left out of a part that holds none of them (`crate::parts`).
                &["app", "level", "creditsNothing"],
            )
        }
        // `sv doctor`'s answers (backlog 0217, part 3).
        "stackvet_status" => object(
            json!({
                "lines": {
                    "type": "array",
                    "items": object(
                        json!({
                            "topic": string,
                            "state": { "type": "string", "enum": ["ready", "not-ready", "cannot-tell"] },
                            "says": string,
                        }),
                        &["topic", "state", "says"],
                    ),
                },
                "notReady": count.clone(),
                // The folder this server serves and the app named there (backlog 0541).
                "root": string,
                "appName": { "type": ["string", "null"] },
                "creditsNothing": { "type": "boolean" },
            }),
            &["lines", "notReady", "root", "appName", "creditsNothing"],
        ),
        "stackvet_preflight" => {
            let items = json!({
                "type": "array",
                "items": object(
                    json!({
                        "topic": string,
                        "answer": { "type": "string", "enum": ["look-at-this", "could-not-tell", "looks-right"] },
                        "says": string,
                    }),
                    &["topic", "answer", "says"],
                ),
            });
            object(
                json!({
                    "ran": { "type": "boolean" },
                    "credits": string,
                    "items": items.clone(),
                    // What `sv run` will check once the app runs, read in the code (ADR-035, Later).
                    "willLookFor": items,
                    // The optional keys of `[stack.run.users]` left out, and what each costs (backlog 0533).
                    "leftOut": {
                        "type": "array",
                        "items": object(
                            json!({
                                "key": string,
                                "checks": string,
                                "requirements": strings,
                            }),
                            &["key", "checks", "requirements"],
                        ),
                    },
                    "notRead": strings,
                }),
                &[
                    "ran",
                    "credits",
                    "items",
                    "willLookFor",
                    "leftOut",
                    "notRead",
                ],
            )
        }
        "stackvet_before" => {
            let item = |fields: &[(&str, Value)]| {
                let properties: serde_json::Map<String, Value> = fields
                    .iter()
                    .map(|(f, kind)| ((*f).to_owned(), kind.clone()))
                    .collect();
                let names: Vec<&str> = fields.iter().map(|(f, _)| *f).collect();
                json!({ "type": "array", "items": object(Value::Object(properties), &names) })
            };
            object(
                json!({
                    "waiting": { "type": "boolean" },
                    "app": string, "level": count, "feature": string, "name": string,
                    "requirements": item(&[("id", string.clone()), ("level", count.clone()), ("description", string.clone())]),
                    "pending": item(&[("id", string.clone()), ("level", count.clone()), ("description", string.clone())]),
                    "conditions": strings,
                    "notApplying": count,
                    "prompts": item(&[("id", string.clone()), ("title", string.clone()), ("status", string.clone()), ("text", string.clone())]),
                    "codingPrompts": item(&[("id", string.clone()), ("title", string.clone()), ("status", string.clone()), ("text", string.clone())]),
                    "rules": item(&[("id", string.clone()), ("topic", string.clone()), ("rule", string.clone())]),
                    "tests": item(&[("id", string.clone()), ("level", count.clone()), ("description", string.clone())]),
                    "settings": item(&[("table", string.clone()), ("key", string.clone()), ("lines", string.clone())]),
                    "creditsNothing": { "type": "boolean" },
                }),
                &[
                    "waiting",
                    "app",
                    "level",
                    "feature",
                    "name",
                    "requirements",
                    "pending",
                    "conditions",
                    "notApplying",
                    "prompts",
                    "codingPrompts",
                    "rules",
                    "tests",
                    "settings",
                    "creditsNothing",
                ],
            )
        }
        "stackvet_prompts" => object(
            json!({
                "prompts": { "type": "array", "items": object(
                    json!({
                        "id": string, "title": string, "prompt": string, "requirements": strings,
                        "sbdControls": strings,
                        "status": { "type": "string", "enum": ["shown", "not-shown", "untested"] },
                        "result": { "type": ["string", "null"] },
                        "forRequirements": strings,
                    }),
                    // With no argument the prompts are listed without their text (backlog 0348).
                    &["id", "title", "requirements", "sbdControls", "status"],
                ) },
                "credit": string,
                "report": string,
                "unproven": count,
            }),
            &["prompts", "credit"],
        ),
        "stackvet_notes_file" => object(
            json!({
                "file": string, "asked": count, "alreadyAnswered": count,
                "keptOutsideQuestions": { "type": "boolean" },
            }),
            &["file", "asked", "alreadyAnswered", "keptOutsideQuestions"],
        ),
        // With an id and an answer, what was recorded; with neither, the file made or refreshed,
        // as `stackvet_notes_file` gave it before it was folded in here.
        "stackvet_record_answer" => object(
            json!({
                "file": string, "id": string, "writtenBy": string,
                "asked": count, "alreadyAnswered": count,
                "keptOutsideQuestions": { "type": "boolean" },
            }),
            &["file"],
        ),
        "stackvet_write_report" => object(json!({ "files": strings }), &["files"]),
        "stackvet_bundle" => object(
            json!({
                "zip": string, "files": count, "appFiles": count,
                "leftOut": { "type": "array", "items": object(json!({ "path": string, "reason": string }), &["path", "reason"]) },
            }),
            &["zip", "files", "appFiles", "leftOut"],
        ),
        "stackvet_explain" => object(
            json!({
                "id": string, "chapter": string, "level": count,
                "levelBasis": { "type": ["string", "null"] },
                "description": string, "counterparts": strings, "couldSettle": string,
            }),
            &[
                "id",
                "chapter",
                "level",
                "levelBasis",
                "description",
                "counterparts",
                "couldSettle",
            ],
        ),
        _ => return None,
    };
    Some(schema)
}

/// The design-time prompts, offered as MCP prompts: what a client shows a person to choose from (in
/// Claude Code, as slash commands), rather than what the model decides to call. Only the design-time
/// file, because these are what to decide before any code, and choosing one is the person's
/// decision; the prompts for the coding stay with `stackvet_prompts` (BACKLOG, "Design-time help
/// before any code", item 2).
pub(super) fn design_prompts() -> Result<sv_check::prompts::Prompts, (i64, String)> {
    let paths = crate::prompts_paths();
    sv_check::prompts::Prompts::load_all(&[&paths[1]]).map_err(|e| (-32603, format!("{e:#}")))
}

/// What the person reads beside a prompt before choosing it: whether it has been shown to work, and
/// which Secure by Design controls it helps them answer. Every copy of a prompt says this, so "not
/// tested" and "shown" never read the same.
pub(super) fn prompt_description(p: &sv_check::prompts::Prompt) -> String {
    let status = p.status_sentence();
    if p.sbd_controls.is_empty() {
        status
    } else {
        format!(
            "{status} Helps you answer Secure by Design {} (you still answer each).",
            p.sbd_controls.join(", ")
        )
    }
}

pub(super) fn prompt_list() -> Result<Value, (i64, String)> {
    let prompts = design_prompts()?;
    let listed: Vec<Value> = prompts
        .prompts
        .iter()
        .map(|p| json!({ "name": p.id, "title": p.title, "description": prompt_description(p) }))
        // After them, the one for reading the report (backlog 0217 part 5).
        .chain(std::iter::once(json!({
            "name": crate::report_prompt::ID,
            "title": crate::report_prompt::TITLE,
            "description": crate::report_prompt::STATUS,
        })))
        .collect();
    Ok(json!({ "prompts": listed }))
}

/// One prompt, as the message the person sends: its text, then whether it was shown to work, that it
/// is an instruction and not evidence, and the credit its license asks for on every copy.
pub(super) fn get_prompt(params: &Value) -> Result<Value, (i64, String)> {
    let name = params.get("name").and_then(Value::as_str).ok_or((
        -32602,
        "prompts/get needs a name, as prompts/list gives".to_owned(),
    ))?;
    if name == crate::report_prompt::ID {
        return Ok(json!({
            "description": crate::report_prompt::STATUS,
            "messages": [{ "role": "user", "content": { "type": "text", "text": crate::report_prompt::with_mark() } }],
        }));
    }
    let prompts = design_prompts()?;
    let Some(p) = prompts.prompts.iter().find(|p| p.id == name) else {
        return Err((-32602, format!("there is no prompt called {name}")));
    };
    let description = prompt_description(p);
    let text = format!(
        "{}\n\n---\n{description} A prompt is an instruction, not evidence: check the app with \
         StackVet afterwards, whichever prompt you use.\n\n{}",
        p.prompt.trim_end(),
        prompts.credit.trim()
    );
    Ok(json!({
        "description": description,
        "messages": [{ "role": "user", "content": { "type": "text", "text": text } }],
    }))
}

pub(super) fn tool_list() -> Value {
    let path = json!({
        "type": "string",
        "description": "The app's folder, relative to the folder this server was started for. Defaults to that folder."
    });
    // For the two tools whose answer can come in parts (`crate::parts`).
    let section = |names: &[&str]| {
        let mut allowed: Vec<&str> = names.to_vec();
        allowed.push("all");
        json!({
            "type": "string",
            "enum": allowed,
            "description": "One section of the answer, as the list at the end of a long answer names them. Leave it out for the whole answer when it is short, or its start and that list when it is long; `all` gives the whole answer whatever its length."
        })
    };
    let page = json!({
        "type": "integer",
        "minimum": 1,
        "description": "Which page of the section, from 1, when the list says it has more than one. Defaults to 1."
    });
    json!([
        {
            "name": "stackvet_status",
            "title": "Is everything ready?",
            "description": "Whether StackVet is ready for this app, one plain line each, marked ready, not ready, or can't tell: which StackVet this is, git, stackvet.toml, how to start the app, and Docker for `sv report --run`. The same answers as `sv doctor` at a terminal. Call it when setting up, or when something does not work.",
            "inputSchema": { "type": "object", "properties": { "path": path.clone() } },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        },
        {
            "name": "stackvet_spec",
            "title": "How to describe the app",
            "description": "The stackvet.toml the app needs before it can be checked, with instructions for filling it in. Write it into the app's folder: before any code, for the app as it will be, decided with the person; once there is code, from what the app really does. A claim the code contradicts is reported, and requirements only ever apply more because of it, never less.",
            "inputSchema": { "type": "object", "properties": {
                "section": section(SPEC_SECTIONS),
                "page": page.clone(),
            } },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        },
        {
            "name": "stackvet_prompts",
            "title": "Prompts for the person to give you",
            "description": "Prompts from StackVet's library for the person to give you: coding prompts for something StackVet checks, and design-time prompts for what to decide before any code, each with the requirements it targets and whether it was shown to work. With no argument it lists them, one line each with its id; ask with `id` for one in full, or with `requirement` or `path` for those that apply. Following one is not evidence of anything, so check the app afterwards.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "requirement": {
                        "type": "string",
                        "description": "Only the prompts for this requirement or Secure by Design control, such as V1.2.4 or SBD-AC-03, each in full. Leave it and `id` out for a list of all of them."
                    },
                    "id": {
                        "type": "string",
                        "description": "One prompt in full, by the id the list gives, such as database-placeholders."
                    },
                    "path": {
                        "type": "string",
                        "description": "The app's folder. Give it for only the prompts for what the app's last report (stackvet-report/report.json, written by stackvet_write_report) shows unproven: a finding, nothing shown, or only an answer given by the person or the AI tool, with nothing checked, each prompt saying which of those requirements it is for. With no report there yet, write one first."
                    }
                }
            },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        },
        {
            "name": "stackvet_plan",
            "title": "Plan the app before writing it",
            "description": "The plan for the app from its stackvet.toml, before any code and at any time after: the requirements that will apply, the design-time prompts, the questions only the person can answer, the tests to write named by requirement id, what `sv run` needs in stackvet.toml, and the threats the answers raise. Built from the same report as stackvet_check; it credits nothing.",
            "inputSchema": { "type": "object", "properties": {
                "path": path.clone(),
                "section": section(crate::plan::SECTIONS),
                "page": page.clone(),
            } },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        },
        {
            "name": "stackvet_before",
            "title": "Before building one feature",
            "description": "Before building one feature: the requirements it brings that apply to this app, the decisions to make first, the coding rules and prompts for it, the tests to write named by requirement id, and the settings `sv run` needs to test it. Asked before stackvet.toml exists, it gives what the feature can bring, and says which requirements apply, and the tests, wait for the file (`waiting`). It credits nothing.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "path": path.clone(),
                    "feature": {
                        "type": "string",
                        "enum": ["sign-in", "sign-in-elsewhere", "admin", "uploads", "payments", "email", "ai", "fetch", "owned-records", "api-keys", "background-jobs", "organizations"],
                        "description": "The feature about to be built."
                    }
                },
                "required": ["feature"]
            },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        },
        {
            "name": "stackvet_guidance",
            "title": "Rules to follow while coding",
            "description": "The security rules to follow while you write this app, adapted from OWASP AISVS 1.0 Appendix C (AI-assisted secure coding), with its attribution and license (CC BY-SA 4.0); rules that do not apply to the app, by its stackvet.toml, are left out. With no topic it ends with the prompts shown to work that are about the whole app. They are instructions, not a check: following them is not evidence of anything.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "path": path.clone(),
                    "topic": {
                        "type": "string",
                        "enum": ["secrets", "untrusted-content", "checking", "review", "dependencies", "agent-limits", "ci-workflows", "provenance", "incidents"],
                        "description": "Only the rules on this topic. Leave it out for all of them."
                    }
                }
            },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        },
        {
            "name": "stackvet_preflight",
            "title": "Will `sv run` be able to test it?",
            "description": "Once there is code: reads the app's files against what stackvet.toml tells `sv run` (the start command, the address and port, the seed, the tables, every path and sign-in field it names) and says for each whether it looks right, needs a look, or could not be told; and what `sv run` will check once the app runs that the code shows no sign of handling. It runs nothing, so \"looks right\" means the text was found, not that it works.",
            "inputSchema": { "type": "object", "properties": { "path": path.clone() } },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        },
        {
            "name": "stackvet_check",
            "title": "Check an app",
            "description": "Check the app against OWASP ASVS 5.0, AISVS 1.0 and the Secure by Design checklist: credentials in the code, configuration, rules that read the code, dependencies (listed, not compared with known vulnerabilities), and which requirements apply. It never starts the app. The result gives the counts, then what was NOT examined, then what needs attention with the file, line and fix, then, in its section \"questions\", what only the person can answer and how to record it. It never says a requirement passed, and nothing in it means the app is secure. Asked again in the same session, the findings it gave before and still finds come a line each.",
            "inputSchema": { "type": "object", "properties": {
                "path": path.clone(),
                "section": section(CHECK_SECTIONS),
                "page": page.clone(),
            } },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        },
        {
            "name": "stackvet_write_report",
            "title": "Write the full report",
            "description": "Write the full reports into a folder inside the app (stackvet-report by default): report.html for a person, compliance.md, security.md, low-findings.md (every low-severity finding in full), stackvet.sarif (the findings, for code-scanning tools), report.json, seen.json (what the running app answered, credentials cut out), sbom.cdx.json (the bill of materials) and report.pdf (the report as one file for the person to print or send). Same checks as stackvet_check.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "path": path,
                    "out": { "type": "string", "description": "Folder inside the app to write to, as a path relative to it: a new or empty one, or one sv wrote before. No `..`. Refused while another run is writing that folder, or when it holds a report from a run that started later." }
                }
            },
            "annotations": { "readOnlyHint": false, "destructiveHint": false, "openWorldHint": false }
        },
        {
            "name": "stackvet_record_answer",
            "title": "Record an answer in the security notes",
            "description": "Write an answer under one question in security-notes.md, making the file if it is not there; with no id and no answer it only makes or refreshes the file, keeping everything already written in it. sv marks every answer this records as yours (`Written by: AI coding tool`), which the report counts for less than the person's own word, and the result says how the person makes it theirs. It fills a question with nothing under it, or replaces an answer marked as yours; anything else under a question may be the person's own words and is never replaced.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "path": path.clone(),
                    "id": { "type": "string", "description": "The question's requirement id, as stackvet_check's section \"questions\" lists it, such as V6.1.1. Leave it and the answer out to only make or refresh the file." },
                    "answer": { "type": "string", "description": "Needed with an id. The answer, in a sentence or two of plain words, at least 40 characters, with no headings and no line starting with `>`. Leave out any line saying who wrote it; sv adds it." }
                }
            },
            "annotations": { "readOnlyHint": false, "destructiveHint": true, "idempotentHint": true, "openWorldHint": false }
        },
        {
            "name": "stackvet_explain",
            "title": "Explain a requirement",
            "description": "What a requirement asks for, in its framework's own words, with its level, where the level comes from, and what kind of run could settle it. Takes an id such as V1.2.4, C9.5.4, AC.4.1 or SBD-AC-05.",
            "inputSchema": {
                "type": "object",
                "properties": { "id": { "type": "string", "description": "The requirement id." } },
                "required": ["id"]
            },
            "annotations": { "readOnlyHint": true, "openWorldHint": false }
        },
        {
            "name": "stackvet_bundle",
            "title": "Bundle the app and its report",
            "description": "Write one zip beside the app (never inside it) holding the app's files, the full report, the bill of materials and a SHA-256 for every file, for the person to keep or hand on. Anything that could hold a secret is left out and listed with the reason; it cannot tell which files hold data about the app's people. The app must be a folder below the one this server was started for.",
            // Its own `path`, required: the zip goes beside the app, so the server's own folder, the
            // default everywhere else, is always refused here (the documentation review, item 5).
            "inputSchema": {
                "type": "object",
                "properties": { "path": {
                    "type": "string",
                    "description": "The app's folder, relative to the folder this server was started for, and below it: the zip is written beside the app, so that folder itself cannot be bundled."
                } },
                "required": ["path"]
            },
            "annotations": { "readOnlyHint": false, "destructiveHint": false, "idempotentHint": true, "openWorldHint": false }
        }
    ])
}

/// The spec's sections, in the order of the whole spec, by the names `section` asks for them.
pub(super) const SPEC_SECTIONS: &[&str] = &["app", "manifest", "instructions", "prompts"];

/// The starter stackvet.toml, how to fill it in, and the prompts shown to work: whole when it fits
/// what an AI coding tool takes in whole, and otherwise in parts (`crate::parts`), first the start
/// of the file, how to fill it in, and the prompts shown to work, which the server's instructions
/// point to here rather than carry (backlog 0338). Measured on 12 October 2026 at 47,187 characters, over the
/// 40,000 an answer is kept to, and each prompt shown to work adds about 800 (backlog 0347).
///
/// It answers in text only, as it always has: the list of parts is in the text, and the
/// structured result `parts` makes for it is not sent, so the tool declares no output shape.
pub(super) fn spec(args: &Value) -> Result<Value> {
    let ask = crate::parts::ask(args, "stackvet_spec", SPEC_SECTIONS)?;
    let whole = spec_text();
    let mut result = crate::parts::respond(
        &crate::parts::Answer {
            tool: "stackvet_spec",
            what: "spec",
            sections: &|_| spec_sections(),
            first: &["app", "instructions", "prompts"],
            only_asked: &[],
            always: serde_json::Map::new(),
            whole_text: &|_| whole.clone(),
            whole_structured: Value::Null,
        },
        &ask,
    )?;
    if let Some(result) = result.as_object_mut() {
        result.remove("structuredContent");
    }
    Ok(result)
}

/// The whole spec, as `sv init` prints it and the tool gave it before it came in parts.
pub(super) fn spec_text() -> String {
    format!(
        "{}\n{}{}",
        sv_manifest::spec::STARTER_MANIFEST,
        sv_manifest::spec::INSTRUCTIONS,
        crate::prompts_at_start()
    )
}

/// The spec cut into sections that join to exactly [`spec_text`]: the start of the file through
/// `[app]`, the rest of it a table to an item, how to fill it in a paragraph to an item, and the
/// prompts one to an item.
fn spec_sections() -> Vec<crate::parts::Section> {
    use crate::parts::{Item, Section};
    /// `text` in items, a new one at each line `starts` says begins one.
    fn cut(text: &str, starts: impl Fn(&str) -> bool) -> Vec<Item> {
        let mut items: Vec<String> = Vec::new();
        for line in text.split_inclusive('\n') {
            match items.last_mut() {
                Some(item) if !starts(line) => item.push_str(line),
                _ => items.push(line.to_owned()),
            }
        }
        items.into_iter().map(Item::text).collect()
    }
    let manifest = format!("{}\n", sv_manifest::spec::STARTER_MANIFEST);
    let at = manifest
        .find("\n[stack]\n")
        .map_or(manifest.len(), |i| i + 1);
    let mut app = Section::new(
        "app",
        "The start of stackvet.toml, through what the app is".to_owned(),
        &[],
    );
    app.lead = manifest[..at].to_owned();
    let mut rest = Section::new(
        "manifest",
        "The rest of stackvet.toml, a table at a time".to_owned(),
        &[],
    );
    rest.items = cut(&manifest[at..], |line| {
        line.starts_with('[') || line.starts_with("# [")
    });
    let mut instructions = Section::new(
        "instructions",
        "How to fill stackvet.toml in".to_owned(),
        &[],
    );
    instructions.items = sv_manifest::spec::INSTRUCTIONS
        .split_inclusive("\n\n")
        .map(|p| Item::text(p.to_owned()))
        .collect();
    let mut prompts = Section::new("prompts", "The prompts shown to work".to_owned(), &[]);
    prompts.items = cut(&crate::prompts_at_start(), |line| line.starts_with("### "));
    vec![app, rest, instructions, prompts]
}

pub(super) fn explain(frameworks: &sv_frameworks::Frameworks, args: &Value) -> Result<Value> {
    let id = args
        .get("id")
        .and_then(Value::as_str)
        .context("stackvet_explain needs an id")?;
    let r = frameworks
        .get(id)
        .with_context(|| format!("{id} is not a requirement in any loaded framework"))?;
    let mut text = format!(
        "{id} ({}), level {}{}\n\n{}",
        r.chapter_name,
        r.level,
        r.level_basis
            .as_deref()
            .map(|b| format!(" — {b}"))
            .unwrap_or_default(),
        r.description
    );
    if !r.counterparts.is_empty() {
        text.push_str(&format!(
            "\n\nASVS requirements that ask the same thing: {}. Evidence about them is shown \
             beside this one as supporting, never as checking it.",
            r.counterparts.join(", ")
        ));
    }
    // What could settle it, so the AI tool can answer "what would it take?" without the terminal (backlog 0426).
    let reach_path = sv_frameworks::data::file("reach.json");
    let reach = std::fs::read_to_string(&reach_path)
        .map_err(anyhow::Error::from)
        .and_then(|text| sv_report::read_reach(&text))
        .with_context(|| format!("reading {}", reach_path.display()))?;
    let could_settle =
        sv_report::could_settle_kinds(reach.requirements.get(id).map_or(&[][..], Vec::as_slice));
    text.push_str(&format!(
        "\n\nWhat could settle it: {could_settle}. That is what could, not a credit: a check settles it only by \
         running and finding it met."
    ));
    Ok(json!({
        "content": [{ "type": "text", "text": text }],
        "structuredContent": {
            "id": id,
            "chapter": r.chapter_name,
            "level": r.level,
            "levelBasis": r.level_basis,
            "description": r.description,
            "counterparts": r.counterparts,
            "couldSettle": could_settle,
        },
        "isError": false,
    }))
}
