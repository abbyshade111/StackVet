//! A small notes service, written to show what StackVet's Rust rules find (backlog 0649). It is read, never built or
//! run. Each fault is on a line marked `// sv:fault <rule>`, and its safe form on one marked `// sv:safe <rule>`; the
//! test in `crates/sv-check/tests/rust_example.rs` holds each rule to the first and never the second.

mod notes;

use axum::extract::{Path, Query};
use axum::Json;
use diesel::prelude::*;
use axum::http::HeaderValue;
use axum::response::{Html, Redirect};
use axum::Router;
use std::process::Command;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::services::ServeDir;

const JWKS_URL: &str = "https://login.example.com/.well-known/jwks.json";
const OWN_SITE: &str = "https://notes.example.com";

// Reading a file at a path a request chose, and at one written out.
async fn download(Path(name): Path<String>) -> String {
    std::fs::read_to_string(&name).unwrap_or_default() // sv:fault file-path-from-request
}

async fn about() -> String {
    std::fs::read_to_string("about.txt").unwrap_or_default() // sv:safe file-path-from-request
}

// Sending a person on to an address the request named, and to one of the app's own.
async fn after_sign_in(q: Query<Next>) -> Redirect {
    Redirect::to(&q.next) // sv:fault open-redirect
}

async fn after_sign_out() -> Redirect {
    Redirect::to("/login") // sv:safe open-redirect
}

// Running a shell on a command built from a value, and a program with its arguments as a list.
fn export(command: &str) {
    Command::new("sh").arg("-c").arg(command).output().ok(); // sv:fault shell-command
}

fn version(program: &str) {
    Command::new(program).arg("--version").output().ok(); // sv:safe shell-command
}

// Serving the app's own folder, and a folder of its assets.
fn routes() -> Router {
    Router::new().nest_service("/", ServeDir::new(".")) // sv:fault static-files-from-app-folder
}

fn asset_routes() -> Router {
    Router::new().nest_service("/assets", ServeDir::new("assets")) // sv:safe static-files-from-app-folder
}

// Connecting to the database as its administrator, and as the app's own account.
const ADMIN_DATABASE: &str = "postgres://postgres@localhost/notes"; // sv:fault database-as-superuser
const APP_DATABASE: &str = "postgres://notes_app@localhost/notes"; // sv:safe database-as-superuser

// Which model answers: one that moves with each release, and one fixed.
fn latest_model() -> &'static str {
    "mistral-small-latest" // sv:fault floating-model-name
}

fn fixed_model() -> &'static str {
    "mistral-small-2503" // sv:safe floating-model-name
}

// The sign-in tokens: their keys, their audience, their signature, and what a failure to read one means.
async fn keys_named_by_the_token(header: Header) {
    let _ = reqwest::get(header.jku.unwrap()).await; // sv:fault token-key-source-from-token
}

async fn keys_of_our_own() {
    let _ = reqwest::get(JWKS_URL).await; // sv:safe token-key-source-from-token
}

fn audience_off() {
    let mut v = Validation::new(Algorithm::RS256);
    v.validate_aud = false; // sv:fault token-audience-not-checked
}

fn audience_on() {
    let mut v = Validation::new(Algorithm::RS256);
    v.set_audience(&["notes"]); // sv:safe token-audience-not-checked
}

fn signature_off() {
    let mut v = Validation::new(Algorithm::HS256);
    v.insecure_disable_signature_validation(); // sv:fault token-signature-not-checked
}

fn signature_on(t: &str, k: &DecodingKey) {
    let _ = decode::<Claims>(t, k, &Validation::new(Algorithm::HS256)); // sv:safe token-signature-not-checked
}

fn verify_lenient(t: &str) -> bool {
    decode::<Claims>(t, &KEY, &V).map(|_| true).unwrap_or(true) // sv:fault check-passes-on-error
}

fn verify_strict(t: &str) -> bool {
    decode::<Claims>(t, &KEY, &V).map(|_| true).unwrap_or(false) // sv:safe check-passes-on-error
}

// A signature compared with `==`, and in constant time.
fn signed_equal(sig: &[u8]) -> bool {
    mac.finalize().into_bytes().as_slice() == sig // sv:fault digest-compared-with-equals
}

fn signed_checked(sig: &[u8]) -> bool {
    mac.verify_slice(sig).is_ok() // sv:safe digest-compared-with-equals
}

// A live connection in plain text, and an encrypted one.
fn plain_socket() {
    let _ = connect_async("ws://chat.example.com/ws"); // sv:fault plaintext-websocket-url
}

fn encrypted_socket() {
    let _ = connect_async("wss://chat.example.com/ws"); // sv:safe plaintext-websocket-url
}

// A link preview that fetches whatever address the request names, and one that asks the app's own preview service
// by id (backlog 0661).
async fn link_preview(q: Query<Preview>) -> String {
    let url = q.url.clone();
    reqwest::get(url).await.map(|_| String::new()).unwrap_or_default() // sv:fault fetch-address-from-request
}

async fn link_preview_by_id(Path(id): Path<u32>) -> String {
    let _ = reqwest::get(&format!("https://preview.example.com/links/{id}")).await; // sv:safe fetch-address-from-request
    String::new()
}

// A greeting page that puts a name from the request into HTML as it came, and one that escapes it first.
async fn greeting(q: Query<Greeting>) -> Html<String> {
    Html(format!("<h1>Hello, {}</h1>", q.name)) // sv:fault html-from-value
}

async fn greeting_escaped(q: Query<Greeting>) -> Html<String> {
    Html(format!("<h1>Hello, {}</h1>", html_escape::encode_text(&q.name))) // sv:safe html-from-value
}

// Cross-site settings: one that lets every site call the app with its visitors' cookies, and one that names the
// app's own site (backlog 0661).
fn cors_any_site() -> CorsLayer {
    CorsLayer::new().allow_origin(AllowOrigin::mirror_request()).allow_credentials(true) // sv:fault cors-any-origin-with-credentials
}

fn cors_own_site() -> CorsLayer {
    CorsLayer::new().allow_origin(OWN_SITE.parse::<HeaderValue>().unwrap()).allow_credentials(true) // sv:safe cors-any-origin-with-credentials
}

// A template made from the request's own text, and one written out with the value passed in as data (backlog 0661).
fn shared_note_page(q: Query<Shared>, ctx: &tera::Context) -> String {
    tera::Tera::one_off(&q.layout, ctx, true).unwrap_or_default() // sv:fault template-built-from-value
}

fn shared_note_page_fixed(ctx: &tera::Context) -> String {
    tera::Tera::one_off("<h1>{{ title }}</h1>", ctx, true).unwrap_or_default() // sv:safe template-built-from-value
}

// A sign-in code made with a generator that can be predicted, and one made with the generator built for secrets
// (backlog 0661).
fn sign_in_code_guessable() -> u32 {
    let sign_in_code = fastrand::u32(100_000..1_000_000); // sv:fault insecure-random-for-code
    sign_in_code
}

fn sign_in_code_secure() -> u32 {
    let sign_in_code = rand::random::<u32>() % 900_000 + 100_000; // sv:safe insecure-random-for-code
    sign_in_code
}

// Saving a profile from the whole request body, so a `role` the form never shows is saved too, and from the fields
// the form may change (backlog 0661).
async fn save_profile(Json(body): Json<Profile>) {
    diesel::insert_into(profiles::table).values(&body).execute(&mut conn()).ok(); // sv:fault request-body-passed-whole
}

async fn save_profile_fields(Json(body): Json<Profile>) {
    let picked = Profile { name: body.name, bio: body.bio, role: Role::Member };
    diesel::insert_into(profiles::table).values(&picked).execute(&mut conn()).ok(); // sv:safe request-body-passed-whole
}

// After "Sign in with" a provider: finding the account by the email address the provider sent, which can pass to
// someone else, and by the provider's own id for the person (backlog 0661).
async fn account_by_provider_email(claims: CoreIdTokenClaims, pool: &SqlitePool) -> Option<User> {
    let email = claims.email()?.as_str().to_owned();
    sqlx::query_as::<_, User>("select * from users where email = ?").bind(email).fetch_optional(pool).await.ok()? // sv:fault account-found-by-provider-email
}

async fn account_by_provider_subject(claims: CoreIdTokenClaims, pool: &SqlitePool) -> Option<User> {
    let subject = claims.subject().as_str().to_owned();
    sqlx::query_as::<_, User>("select * from users where sub = ?").bind(subject).fetch_optional(pool).await.ok()? // sv:safe account-found-by-provider-email
}

// Saving a person's backup codes as they were made, which anyone reading the database can use, and saving only a
// hash of each (backlog 0661).
fn save_backup_codes(account: &mut Account) {
    account.backup_codes = (0..10).map(|_| rand::random::<u64>().to_string()).collect(); // sv:fault backup-codes-stored-plain
}

fn save_backup_codes_hashed(account: &mut Account) {
    account.backup_codes = (0..10).map(|_| bcrypt::hash(rand::random::<u64>().to_string(), 12).unwrap()).collect(); // sv:safe backup-codes-stored-plain
}

// A rule that makes passwords expire on a schedule, which leads people to weaker ones, and the length rule beside it
// (backlog 0661).
const PASSWORD_EXPIRY_DAYS: u32 = 90; // sv:fault password-expiry-enforced
const PASSWORD_MIN_LENGTH: usize = 12; // sv:safe password-expiry-enforced

#[cfg(test)]
mod tests {
    // In test code: still found, and listed apart (ADR-023).
    #[test]
    fn a_digest() {
        let _ = md5::compute(b"fixture"); // sv:test weak-hash-function
    }
}
