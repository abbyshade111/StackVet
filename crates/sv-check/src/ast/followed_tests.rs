//! Backlog 0582: `ast.file-path-from-value` follows a name in a Python path to where the same file sets it. A module
//! constant, a name made with `secrets.token_hex` or `uuid4`, a `secure_filename` result kept in a name, and a
//! parameter checked narrowly before the call are not reported; a value from a request still is, by name or not.

use super::*;

const RULE: &str = "ast.file-path-from-value";

fn rules() -> &'static AstRules {
    static RULES: std::sync::OnceLock<AstRules> = std::sync::OnceLock::new();
    RULES.get_or_init(|| {
        AstRules::load(
            &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/ast-rules.json"),
        )
        .expect("rules load")
    })
}

fn reported(source: &str) -> bool {
    scan_file(rules(), "python", "games.py", source)
        .iter()
        .any(|f| f.rule_id == RULE)
}

#[test]
fn a_module_constant_built_from_the_app_folder_is_not_reported() {
    assert!(!reported(
        "import os\ngames_file = os.path.join(os.path.dirname(__file__), 'games.json')\n\
         def load():\n    with open(games_file) as f:\n        return f.read()\n"
    ));
}

#[test]
fn a_name_made_with_token_hex_or_uuid4_is_not_reported() {
    assert!(!reported(
        "import os, secrets\nUPLOADS = 'uploads'\ndef save(data):\n    name = secrets.token_hex(16) + '.png'\n\
         \x20   with open(os.path.join(UPLOADS, name), 'wb') as f:\n        f.write(data)\n"
    ));
    assert!(!reported(
        "import os, uuid\nUPLOADS = 'uploads'\ndef save(data):\n    name = f'{uuid.uuid4().hex}.png'\n\
         \x20   with open(os.path.join(UPLOADS, name), 'wb') as f:\n        f.write(data)\n"
    ));
}

#[test]
fn a_secure_filename_kept_in_a_name_is_not_reported() {
    assert!(!reported(
        "import os\nfrom werkzeug.utils import secure_filename\nUPLOADS = 'uploads'\ndef save(f):\n\
         \x20   name = secure_filename(f.filename)\n    path = os.path.join(UPLOADS, name)\n    return open(path).read()\n"
    ));
}

#[test]
fn a_parameter_checked_narrowly_before_the_call_is_not_reported() {
    assert!(!reported(
        "import os, re\nUPLOADS = 'uploads'\ndef load(name):\n    if not re.fullmatch(r'[a-z0-9_-]+', name):\n\
         \x20       abort(400)\n    with open(os.path.join(UPLOADS, name + '.json')) as f:\n        return f.read()\n"
    ));
}

#[test]
fn a_value_from_a_request_is_still_reported_by_name_or_not() {
    // The setup: the rule still reports the plain case.
    assert!(reported(
        "import os\nfrom flask import request\ndef load():\n    with open(os.path.join('uploads', request.args['name'])) as f:\n\
         \x20       return f.read()\n"
    ));
    // Kept in a name first.
    assert!(reported(
        "import os\nfrom flask import request\ndef load():\n    name = request.args['name']\n\
         \x20   with open(os.path.join('uploads', name)) as f:\n        return f.read()\n"
    ));
    // A safe value on one path and a request's on another: not every value is safe.
    assert!(reported(
        "import os, secrets\nfrom flask import request\ndef save(data, mine):\n    name = secrets.token_hex(8)\n\
         \x20   if not mine:\n        name = request.args['name']\n    open(os.path.join('uploads', name), 'wb').write(data)\n"
    ));
}

#[test]
fn a_parameter_checked_loosely_or_not_stopped_is_still_reported() {
    // The check lets a `.` through.
    assert!(reported(
        "import os, re\ndef load(name):\n    if not re.fullmatch(r'.+', name):\n        abort(400)\n\
         \x20   return open(os.path.join('uploads', name)).read()\n"
    ));
    // A check that does not stop the function.
    assert!(reported(
        "import os, re\ndef load(name):\n    if not re.fullmatch(r'[a-z]+', name):\n        print('odd')\n\
         \x20   return open(os.path.join('uploads', name)).read()\n"
    ));
    // Unchecked.
    assert!(reported(
        "import os\ndef load(name):\n    return open(os.path.join('uploads', name)).read()\n"
    ));
}

#[test]
fn a_module_name_a_function_sets_from_a_request_is_still_reported() {
    assert!(reported(
        "import os\nfrom flask import request\ncurrent = 'a.json'\ndef pick():\n    global current\n\
         \x20   current = request.args['f']\ndef load():\n    return open(current).read()\n"
    ));
}
