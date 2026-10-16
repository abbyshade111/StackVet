//! Backlog 0608: a check applied to every route at once, Django's `urls.py` with its views, and Go's handlers.

use super::{EVERY_ROUTE, RouteLine, in_source};
use crate::test_support::scratch;

fn route(file: &str, line: usize, method: &str, path: &str, guards: &[&str]) -> RouteLine {
    RouteLine {
        file: file.to_owned(),
        line,
        method: method.to_owned(),
        path: path.to_owned(),
        guards: guards.iter().map(|g| (*g).to_owned()).collect(),
    }
}

#[test]
fn express_middleware_named_as_a_check_is_a_row_and_other_middleware_is_not() {
    let source = "\
app.use(express.json());
app.use(cors());
app.use(requireAuth);
app.use('/admin', ensureAdmin, adminRouter);
router.use(passport.authenticate('jwt'));
app.use('/api', apiRouter);
app.get('/notes', listNotes);
";
    assert_eq!(
        in_source("javascript", "server.js", source),
        vec![
            route("server.js", 3, EVERY_ROUTE, "*", &["requireAuth"]),
            route("server.js", 4, EVERY_ROUTE, "/admin/*", &["ensureAdmin"]),
            route(
                "server.js",
                5,
                EVERY_ROUTE,
                "*",
                &["passport.authenticate('jwt')"]
            ),
            route("server.js", 7, "GET", "/notes", &[]),
        ]
    );
}

#[test]
fn go_handlers_with_their_wrappers_and_router_middleware() {
    let source = "\
package main

func main() {
\thttp.HandleFunc(\"/\", home)
\tmux.Handle(\"/admin\", requireAuth(adminOnly(http.HandlerFunc(admin))))
\tmux.HandleFunc(\"POST /notes\", createNote)
\tr.GET(\"/account\", authRequired(), account)
\tr.Get(\"/health\", health)
\tr.Use(middleware.Logger)
\tr.Use(jwtauth.Verifier(tokenAuth))
\tfmt.Println(\"/not-a-route\")
}
";
    assert_eq!(
        in_source("go", "main.go", source),
        vec![
            route("main.go", 4, "ANY", "/", &[]),
            route("main.go", 5, "ANY", "/admin", &["requireAuth", "adminOnly"]),
            route("main.go", 6, "POST", "/notes", &[]),
            route("main.go", 7, "GET", "/account", &["authRequired()"]),
            route("main.go", 8, "GET", "/health", &[]),
            route(
                "main.go",
                10,
                EVERY_ROUTE,
                "*",
                &["jwtauth.Verifier(tokenAuth)"]
            ),
        ]
    );
}

#[test]
fn django_routes_carry_what_is_written_on_their_views_in_another_file() {
    let dir = scratch("");
    std::fs::create_dir_all(dir.join("notes")).unwrap();
    std::fs::write(
        dir.join("notes/views.py"),
        "\
from django.contrib.auth.decorators import login_required

def home(request):
    pass

@login_required
@permission_required('notes.delete_note')
def delete_note(request, pk):
    pass

class NoteList(LoginRequiredMixin, ListView):
    pass

class NoteApi(APIView):
    permission_classes = [IsAuthenticated]
",
    )
    .unwrap();
    std::fs::write(
        dir.join("notes/urls.py"),
        "\
from django.urls import path, include
from . import views

urlpatterns = [
    path('', views.home),
    path('notes/<int:pk>/delete/', views.delete_note),
    path('notes/', views.NoteList.as_view()),
    path('api/notes/', views.NoteApi.as_view()),
    path('secret/', login_required(views.home)),
    path('accounts/', include('allauth.urls')),
]
",
    )
    .unwrap();
    let listing = sv_scan::files::Listing::of(&dir);
    // The setup worked: both files were listed as the app's own.
    assert_eq!(listing.app_files().count(), 2);
    let routes = super::read_all(&listing);
    let file = "notes/urls.py";
    assert_eq!(
        routes,
        vec![
            route(file, 5, "ANY", "/", &[]),
            route(
                file,
                6,
                "ANY",
                "/notes/<int:pk>/delete/",
                &["login_required", "permission_required('notes.delete_note')"]
            ),
            route(file, 7, "ANY", "/notes/", &["LoginRequiredMixin"]),
            route(
                file,
                8,
                "ANY",
                "/api/notes/",
                &["permission_classes = [IsAuthenticated]"]
            ),
            route(file, 9, "ANY", "/secret/", &["login_required"]),
        ]
    );
}
