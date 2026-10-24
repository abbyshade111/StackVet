//! Backlog 0666: three ways a framework writes a fault that the code rules did not read, found when
//! `data/stack-guidance.json` (ADR-143) had to drop the line for each. FastAPI's `CORSMiddleware`
//! letting every site in with credentials, sqlx's `sqlx::query(&format!(…))`, and Gin's and Echo's
//! `c.Redirect(code, address)`. Each has its fault and its safe form here.

use super::*;
use std::path::PathBuf;

fn rules() -> &'static AstRules {
    static RULES: OnceLock<AstRules> = OnceLock::new();
    RULES.get_or_init(|| {
        AstRules::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"))
            .expect("rules load")
    })
}

/// Whether `rule` reports `source`. A file that did not parse, or a query that did not compile,
/// fails the test rather than reading as clean.
fn reported(rule: &str, language: &str, file: &str, source: &str) -> bool {
    let read = read_file(rules(), language, file, source);
    assert!(!read.parse_error, "{file} did not parse: {source}");
    assert!(
        read.broken.is_empty(),
        "a query did not compile: {:?}",
        read.broken
    );
    read.findings.iter().any(|f| f.rule_id == rule)
}

fn each(rule: &str, language: &str, file: &str, cases: &[(&str, bool)]) {
    for (source, expected) in cases {
        assert_eq!(
            reported(rule, language, file, source),
            *expected,
            "{rule} on:\n{source}"
        );
    }
}

#[test]
fn fastapis_cors_middleware_with_every_site_and_credentials_is_found() {
    each(
        "ast.cors-any-origin-with-credentials",
        "python",
        "main.py",
        &[
            (
                "app = FastAPI()\napp.add_middleware(CORSMiddleware, allow_origins=[\"*\"], allow_credentials=True)\n",
                true,
            ),
            (
                "app.add_middleware(\n    CORSMiddleware,\n    allow_credentials=True,\n    allow_methods=[\"*\"],\n    allow_origins=['*'],\n)\n",
                true,
            ),
            (
                "app.add_middleware(CORSMiddleware, allow_origin_regex=\".*\", allow_credentials=True)\n",
                true,
            ),
            // Starlette's own way: the middleware listed when the app is made.
            (
                "app = Starlette(middleware=[Middleware(CORSMiddleware, allow_origins=[\"*\"], allow_credentials=True)])\n",
                true,
            ),
            // A list of sites, or no credentials, is the safe form.
            (
                "app.add_middleware(CORSMiddleware, allow_origins=[\"https://app.example.com\"], allow_credentials=True)\n",
                false,
            ),
            (
                "app.add_middleware(CORSMiddleware, allow_origins=[\"*\"])\n",
                false,
            ),
            (
                "app.add_middleware(CORSMiddleware, allow_origins=[\"*\"], allow_credentials=False)\n",
                false,
            ),
            // Another middleware with the same words is not CORS.
            (
                "app.add_middleware(SessionMiddleware, allow_origins=[\"*\"], allow_credentials=True)\n",
                false,
            ),
        ],
    );
}

#[test]
fn sqlxs_query_functions_given_text_built_with_format_are_found() {
    each(
        "ast.sql-built-by-hand",
        "rust",
        "src/db.rs",
        &[
            (
                "async fn find(pool: &PgPool, who: &str) {\n    sqlx::query(&format!(\"SELECT * FROM notes WHERE owner = '{}'\", who)).fetch_all(pool).await;\n}\n",
                true,
            ),
            (
                "async fn find(pool: &PgPool, who: &str) {\n    sqlx::query_as::<_, Note>(&format!(\"SELECT * FROM notes WHERE owner = '{who}'\")).fetch_all(pool).await;\n}\n",
                true,
            ),
            (
                "async fn count(pool: &PgPool, t: &str) {\n    sqlx::query_scalar(&format!(\"SELECT count(*) FROM {t}\")).fetch_one(pool).await;\n}\n",
                true,
            ),
            // The safe form: fixed text, with the value bound.
            (
                "async fn find(pool: &PgPool, who: &str) {\n    sqlx::query(\"SELECT * FROM notes WHERE owner = $1\").bind(who).fetch_all(pool).await;\n}\n",
                false,
            ),
            (
                "async fn find(pool: &PgPool, who: &str) {\n    sqlx::query_as::<_, Note>(\"SELECT * FROM notes WHERE owner = $1\").bind(who).fetch_all(pool).await;\n}\n",
                false,
            ),
            // A query parameter type of a web framework is not a database query.
            (
                "async fn page(q: web::Query<Params>) -> String {\n    format!(\"{}\", q.page)\n}\n",
                false,
            ),
        ],
    );
}

#[test]
fn gins_and_echos_redirect_to_an_address_from_the_request_is_found() {
    each(
        "ast.open-redirect",
        "go",
        "main.go",
        &[
            (
                "package main\nfunc back(c *gin.Context) {\n\tc.Redirect(302, c.Query(\"to\"))\n}\n",
                true,
            ),
            (
                "package main\nfunc back(c *gin.Context) {\n\tc.Redirect(http.StatusFound, c.PostForm(\"next\"))\n}\n",
                true,
            ),
            (
                "package main\nfunc back(c echo.Context) error {\n\treturn c.Redirect(http.StatusSeeOther, c.QueryParam(\"to\"))\n}\n",
                true,
            ),
            (
                "package main\nfunc back(ctx *gin.Context) {\n\tctx.Redirect(302, ctx.Query(\"to\"))\n}\n",
                true,
            ),
            // The safe form: a fixed path of the app's own.
            (
                "package main\nfunc back(c *gin.Context) {\n\tc.Redirect(302, \"/login\")\n}\n",
                false,
            ),
            // net/http's form still read as before, and something else's Redirect still not.
            (
                "package main\nfunc back(w http.ResponseWriter, r *http.Request) {\n\thttp.Redirect(w, r, r.URL.Query().Get(\"next\"), http.StatusFound)\n}\n",
                true,
            ),
            (
                "package main\nfunc f(cache *Cache, next string) {\n\tcache.Redirect(next, 302)\n}\n",
                false,
            ),
        ],
    );
}
