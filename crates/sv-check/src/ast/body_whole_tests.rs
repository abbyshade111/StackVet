//! `ast.request-body-passed-whole` (the gap analysis of 7 October 2026, finding 11): a record created
//! or updated from the whole request body, so any field the request names is written (mass
//! assignment, V15.3.3). Its own file, so its cases do not land at the end of `tests.rs`.

use super::*;
use std::path::PathBuf;

const RULE: &str = "ast.request-body-passed-whole";

fn rules() -> &'static AstRules {
    static RULES: OnceLock<AstRules> = OnceLock::new();
    RULES.get_or_init(|| {
        AstRules::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"))
            .expect("rules load")
    })
}

/// Whether the rule reports `source`, read as `file`. A file the parser could not read fails the
/// test rather than reading as clean.
fn reported(language: &str, file: &str, source: &str) -> bool {
    let read = read_file(rules(), language, file, source);
    assert!(!read.parse_error, "{file} did not parse: {source}");
    assert!(
        read.broken.is_empty(),
        "a query did not compile: {:?}",
        read.broken
    );
    read.findings.iter().any(|f| f.rule_id == RULE)
}

const JAVASCRIPT: &[(&str, bool)] = &[
    // The body through a name (backlog 0661, `argumentNamesWhole`); a field picked out of it is not the body.
    (
        "app.post('/u', async (req, res) => { const body = req.body; res.json(await User.create(body)); });",
        true,
    ),
    (
        "app.post('/u', async (req, res) => { const body = req.body; res.json(await User.create(body.name)); });",
        false,
    ),
    (
        "app.post('/u', async (req, res) => res.json(await User.create(req.body)));",
        true,
    ),
    (
        "app.put('/u/:id', async (req, res) => res.json(await user.update(req.body)));",
        true,
    ),
    (
        "app.put('/u/:id', async (req, res) => res.json(await User.findByIdAndUpdate(req.params.id, req.body)));",
        true,
    ),
    (
        "app.post('/u', async (req, res) => res.json(await prisma.user.create({ data: req.body })));",
        true,
    ),
    (
        "app.post('/u', async (req, res) => { const u = new User(req.body); await u.save(); res.json(u); });",
        true,
    ),
    (
        "router.post('/u', async (ctx) => { ctx.body = await User.create(ctx.request.body); });",
        true,
    ),
    // The fields picked out, one of them, or the body checked by a schema first.
    (
        "app.post('/u', async (req, res) => res.json(await User.create({ name: req.body.name, email: req.body.email })));",
        false,
    ),
    (
        "app.put('/u/:id', async (req, res) => res.json(await user.update({ name: req.body.name })));",
        false,
    ),
    (
        "app.post('/u', async (req, res) => res.json(await User.create(schema.parse(req.body))));",
        false,
    ),
    (
        "app.post('/u', async (req, res) => res.json(await prisma.user.create({ data: { name: req.body.name } })));",
        false,
    ),
    // The id is the first argument of `findByIdAndUpdate`, and it is not the body.
    (
        "app.put('/u/:id', async (req, res) => res.json(await User.findByIdAndUpdate(req.body.id, { seen: true })));",
        false,
    ),
    // Not a create or update.
    ("app.post('/u', (req, res) => res.json(req.body));", false),
];

const PYTHON: &[(&str, bool)] = &[
    (
        "def create(request):\n    user = User(**request.json)\n    db.session.add(user)\n",
        true,
    ),
    (
        "def create(request):\n    return Note.objects.create(**request.data)\n",
        true,
    ),
    (
        "def edit(request, note):\n    Note.objects.filter(id=note).update(**request.get_json())\n",
        true,
    ),
    (
        "def create(request):\n    return Note.objects.create(**request.POST.dict())\n",
        true,
    ),
    // The fields picked out, or checked first.
    (
        "def create(request):\n    return Note.objects.create(title=request.data['title'])\n",
        false,
    ),
    (
        "def create(request):\n    data = UserIn.parse_obj(request.json)\n    return User(name=data.name, email=data.email)\n",
        false,
    ),
    // A schema class built the same way as a model looks just like one, and is reported too: the
    // limit the design entry names.
    (
        "def create(request):\n    return UserIn(**request.json)\n",
        true,
    ),
    (
        "def create(request, defaults):\n    return Note.objects.create(**defaults)\n",
        false,
    ),
];

// Rust (backlog 0661): diesel's `values` and `set`, and sea-orm's `from_json`, given the whole body an Axum or Actix
// `Json` or `Form` extractor read.
const RUST: &[(&str, bool)] = &[
    (
        "async fn create(Json(body): Json<NewUser>) { diesel::insert_into(users::table).values(&body).execute(&mut conn()).unwrap(); }\n",
        true,
    ),
    (
        "async fn update(Path(id): Path<i32>, Json(changes): Json<UserChanges>) { diesel::update(users::table.find(id)).set(&changes).execute(&mut conn()).unwrap(); }\n",
        true,
    ),
    (
        "async fn create(web::Json(body): web::Json<serde_json::Value>) { let user = user::ActiveModel::from_json(body).unwrap(); user.insert(&db()).await.unwrap(); }\n",
        true,
    ),
    (
        "async fn create(Form(form): Form<NewUser>) { let new = form; diesel::insert_into(users::table).values(&new).execute(&mut conn()).unwrap(); }\n",
        true,
    ),
    // The fields picked out, the owner set by the app; a value the app built; a query's own filter.
    (
        "async fn create(Json(body): Json<NewUser>) { let new = NewUser { name: body.name, email: body.email, role: Role::Member }; diesel::insert_into(users::table).values(&new).execute(&mut conn()).unwrap(); }\n",
        false,
    ),
    (
        "async fn create(Json(body): Json<NewUser>) { diesel::insert_into(users::table).values((users::name.eq(&body.name), users::email.eq(&body.email))).execute(&mut conn()).unwrap(); }\n",
        false,
    ),
    (
        "fn seed() { diesel::insert_into(users::table).values(&default_admin()).execute(&mut conn()).unwrap(); }\n",
        false,
    ),
    (
        "async fn show(Query(q): Query<Find>) { let _ = map.values(); let _ = headers.set(&q); }\n",
        false,
    ),
];

#[test]
fn the_whole_body_saved_is_reported_and_the_fields_picked_out_are_not() {
    let mut wrong = Vec::new();
    for (language, file, cases) in [
        ("javascript", "app.js", JAVASCRIPT),
        ("typescript", "app.ts", JAVASCRIPT),
        ("python", "views.py", PYTHON),
        ("rust", "src/main.rs", RUST),
    ] {
        for (source, expected) in cases {
            if reported(language, file, source) != *expected {
                wrong.push(format!("{language}, expected {expected}: {source}"));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn the_rule_is_only_ever_a_finding() {
    let rule = rules()
        .rules()
        .find(|r| r.id == RULE)
        .expect("the rule is loaded");
    assert!(rule.findings_only, "finding none credits nothing");
    assert_eq!(rule.requirement_ids, ["V15.3.3"]);
}
