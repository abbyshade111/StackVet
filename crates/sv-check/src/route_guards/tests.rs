//! What the route reader takes as a route, and what it lists as written on it (backlog 0493).

use super::{RouteLine, in_source};

fn route(line: usize, method: &str, path: &str, guards: &[&str]) -> RouteLine {
    RouteLine {
        file: "app.py".to_owned(),
        line,
        method: method.to_owned(),
        path: path.to_owned(),
        guards: guards.iter().map(|g| (*g).to_owned()).collect(),
    }
}

#[test]
fn flask_routes_with_and_without_a_guard() {
    let source = "\
@app.route(\"/\")
def home():
    return 'hi'

@app.route(\"/admin\", methods=[\"GET\", \"POST\"])
@login_required
@roles_required('admin')
def admin():
    pass

@bp.post('/notes/<int:id>/delete')
def delete(id):
    pass

@app.before_request
def load_user():
    pass
";
    assert_eq!(
        in_source("python", "app.py", source),
        vec![
            route(1, "GET", "/", &[]),
            route(
                5,
                "GET, POST",
                "/admin",
                &["login_required", "roles_required('admin')"]
            ),
            route(11, "POST", "/notes/<int:id>/delete", &[]),
            // A hook run before every request is a row of its own (backlog 0608).
            route(15, "EVERY ROUTE", "*", &["load_user"]),
        ]
    );
}

#[test]
fn fastapi_dependencies_are_guards() {
    let source = "\
@router.get(\"/me\")
async def me(user: User = Depends(current_user)):
    return user

@router.delete(\"/items/{id}\", dependencies=[Depends(require_admin)])
async def remove(id: int, db = Depends(get_db)):
    pass

@app.get(\"/healthz\")
def healthz():
    return {}
";
    assert_eq!(
        in_source("python", "app.py", source),
        vec![
            route(1, "GET", "/me", &["Depends(current_user)"]),
            route(
                5,
                "DELETE",
                "/items/{id}",
                &["Depends(require_admin)", "Depends(get_db)"]
            ),
            route(9, "GET", "/healthz", &[]),
        ]
    );
}

#[test]
fn express_routes_list_what_runs_before_the_handler() {
    let source = "\
const app = express();
app.get('/', (req, res) => res.send('hi'));
app.post('/notes', requireAuth, csrf, async (req, res) => {});
adminRouter.delete(`/users/:id`, requireRole('admin'), handler);
router.get('/public', handler);
";
    let found: Vec<RouteLine> = in_source("javascript", "server.js", source)
        .into_iter()
        .map(|mut r| {
            r.file = "app.py".to_owned();
            r
        })
        .collect();
    assert_eq!(
        found,
        vec![
            route(2, "GET", "/", &[]),
            route(3, "POST", "/notes", &["requireAuth", "csrf"]),
            route(4, "DELETE", "/users/:id", &["requireRole('admin')"]),
            route(5, "GET", "/public", &[]),
        ]
    );
}

#[test]
fn what_is_not_a_route_is_not_listed() {
    // A client's request, a map's `get`, a path that is not a literal, a route with no handler.
    let script = "\
axios.get('/api/notes', config);
cache.get('/key', fallback);
app.get(somePath, handler);
app.get(`/notes/${id}`, handler);
app.get('/only-a-path');
app.get('setting-name');
";
    assert_eq!(in_source("javascript", "client.js", script), Vec::new());
    let python = "\
@functools.lru_cache(maxsize=None)
def cached():
    pass

@app.get(PATH)
def dynamic():
    pass

@app.get(f\"/{name}\")
def formatted():
    pass

@property
def value(self):
    pass
";
    assert_eq!(in_source("python", "app.py", python), Vec::new());
}
