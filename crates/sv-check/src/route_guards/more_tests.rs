//! Backlog 0608, second part: Spring, ASP.NET, Laravel, and Rails.

use super::{RouteLine, in_source};
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
fn spring_mappings_with_the_method_and_class_guards() {
    let source = "\
@RestController
@RequestMapping(\"/api\")
@PreAuthorize(\"hasRole('USER')\")
class NoteController {
  @GetMapping(\"/notes\")
  @Secured(\"ROLE_ADMIN\")
  List<Note> list() { return null; }

  @RequestMapping(value = \"/import\", method = RequestMethod.POST)
  void importNotes() {}

  void helper() {}
}
";
    assert_eq!(
        in_source("java", "NoteController.java", source),
        vec![
            route(
                "NoteController.java",
                5,
                "GET",
                "/api/notes",
                &[
                    "@Secured(\"ROLE_ADMIN\")",
                    "on the class: @PreAuthorize(\"hasRole('USER')\")"
                ]
            ),
            route(
                "NoteController.java",
                9,
                "POST",
                "/api/import",
                &["on the class: @PreAuthorize(\"hasRole('USER')\")"]
            ),
        ]
    );
}

#[test]
fn aspnet_attributes_and_minimal_api() {
    let source = "\
[Authorize(Roles = \"Admin\")]
[Route(\"api/[controller]\")]
public class NotesController : ControllerBase {
  [HttpGet(\"{id}\")]
  [AllowAnonymous]
  public IActionResult Get(int id) { return Ok(); }

  public IActionResult Helper() { return Ok(); }
}
class Program {
  void Main() {
    app.MapGet(\"/health\", () => \"ok\");
    app.MapPost(\"/admin\", () => 1).RequireAuthorization(\"admin\");
  }
}
";
    assert_eq!(
        in_source("csharp", "Notes.cs", source),
        vec![
            route(
                "Notes.cs",
                4,
                "GET",
                "/api/[controller]/{id}",
                &[
                    "AllowAnonymous",
                    "on the class: Authorize(Roles = \"Admin\")"
                ]
            ),
            route("Notes.cs", 12, "GET", "/health", &[]),
            route(
                "Notes.cs",
                13,
                "POST",
                "/admin",
                &["RequireAuthorization(\"admin\")"]
            ),
        ]
    );
}

#[test]
fn laravel_routes_with_their_middleware_and_their_group() {
    let source = "\
<?php
Route::get('/notes', [NoteController::class, 'index'])->middleware('auth');
Route::get('about', fn () => view('about'));
Route::middleware(['auth', 'can:admin'])->group(function () {
    Route::post('/admin', [AdminController::class, 'store']);
});
";
    assert_eq!(
        in_source("php", "routes/web.php", source),
        vec![
            route(
                "routes/web.php",
                2,
                "GET",
                "/notes",
                &["middleware('auth')"]
            ),
            route("routes/web.php", 3, "GET", "/about", &[]),
            route(
                "routes/web.php",
                5,
                "POST",
                "/admin",
                &["in a group: middleware(['auth', 'can:admin'])"]
            ),
        ]
    );
}

#[test]
fn rails_routes_carry_their_controllers_before_actions() {
    let dir = scratch("");
    std::fs::create_dir_all(dir.join("config")).unwrap();
    std::fs::create_dir_all(dir.join("app/controllers")).unwrap();
    std::fs::write(
        dir.join("config/routes.rb"),
        "\
Rails.application.routes.draw do
  root 'home#index'
  get '/notes', to: 'notes#index'
  post 'login' => 'sessions#create'
  resources :notes, only: [:index, :show]
end
",
    )
    .unwrap();
    std::fs::write(
        dir.join("app/controllers/application_controller.rb"),
        "class ApplicationController < ActionController::Base\n  before_action :set_locale\nend\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("app/controllers/notes_controller.rb"),
        "class NotesController < ApplicationController\n  before_action :authenticate_user!, only: [:index]\nend\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("app/controllers/sessions_controller.rb"),
        "class SessionsController < ApplicationController\n  skip_before_action :verify_authenticity_token\nend\n",
    )
    .unwrap();
    let listing = sv_scan::files::Listing::of(&dir);
    // The setup worked: every file was listed as the app's own.
    assert_eq!(listing.app_files().count(), 4);
    let file = "config/routes.rb";
    let notes = [
        "in ApplicationController: before_action :set_locale",
        "before_action :authenticate_user!, only: [:index]",
    ];
    assert_eq!(
        super::read_all(&listing),
        vec![
            route(file, 2, "GET", "/", &[]),
            route(file, 3, "GET", "/notes", &notes),
            route(
                file,
                4,
                "POST",
                "/login",
                &[
                    "in ApplicationController: before_action :set_locale",
                    "skip_before_action :verify_authenticity_token"
                ]
            ),
            route(file, 5, "RESOURCES", "/notes", &notes),
        ]
    );
}
