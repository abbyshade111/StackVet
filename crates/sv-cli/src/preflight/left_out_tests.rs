//! Backlog 0533: preflight lists, in a table of its own, each optional key of `[stack.run.users]` the manifest leaves
//! out, the checks that will not run, and the requirements they speak to. In the three-tool prompt trial of 14 October
//! 2026, two of three apps left `owned` out and preflight said nothing. Not something `sv run` needs, so the count of
//! what to look at is unchanged.

use super::*;

fn manifest(users: &str) -> Manifest {
    let text = format!(
        "manifest-version = 1\n[app]\nname = \"t\"\n[stack.run]\nimage = \"python:3.12-slim\"\n\
         start = \"python app.py\"\nhealth = \"/health\"\n{users}"
    );
    Manifest::parse(&text, Path::new("stackvet.toml")).expect("the test's manifest parses")
}

const USERS: &str = "[stack.run.users]\nseed = \"python seed.py\"\n\
                     login = { path = \"/login\", form = { email = \"{user}\", password = \"{password}\" } }\n\
                     private = [\"/account\"]\nlogout = { path = \"/logout\" }\n";

#[test]
fn a_manifest_without_owned_is_told_what_that_costs_in_a_table_of_its_own() {
    let m = manifest(USERS);
    let left = left_out(&m);
    let keys: Vec<&str> = left.iter().map(|r| r.key).collect();
    // The setup: what this manifest leaves out, and not what it sets.
    assert_eq!(keys, ["owned", "admin", "admin-actions", "once", "signup"]);
    let text = markdown(&[], &[], &left, &[]);
    assert!(
        text.contains("## What leaving a key out of `[stack.run.users]` costs"),
        "{text}"
    );
    assert!(
        text.contains(
            "| `owned` | reading one user's record as another user, a forged form sent from another site, the limit \
             on making records, which fields a record read back hands over, and saved text written back into a page \
             unencoded | V8.2.2, V3.5.1, V3.5.2, V2.4.1, V15.3.1, V8.2.3, V1.2.1 |"
        ),
        "{text}"
    );
    assert!(!text.contains("| `logout` |"), "{text}");
    // Not counted with what `sv run` needs.
    assert!(text.contains("\n0 to look at,"), "{text}");
    let json = to_json(&[], &[], &left, &[]);
    let rows = json["leftOut"].as_array().expect("leftOut is a list");
    assert_eq!(rows.len(), 5);
    assert_eq!(rows[0]["key"], "owned");
    assert_eq!(rows[0]["requirements"][1], "V3.5.1");
}

#[test]
fn nothing_is_left_out_without_users_or_with_every_key() {
    assert!(left_out(&manifest("")).is_empty());
    let every = format!(
        "{USERS}signup = {{ path = \"/signup\", form = {{ email = \"{{user}}\", password = \"{{password}}\" }} }}\n\
         admin = [\"/admin\"]\nadmin-actions = [{{ method = \"POST\", path = \"/admin/ban\" }}]\n\
         once = {{ path = \"/book\", completed = \"Booked\" }}\n\
         owned = {{ create = {{ path = \"/notes\", form = {{ text = \"{{marker}}\" }} }}, \
         delete = {{ path = \"/notes/{{id}}/delete\" }} }}\n"
    );
    let m = manifest(&every);
    assert!(left_out(&m).is_empty(), "{:?}", left_out(&m));
    let text = markdown(&[], &[], &left_out(&m), &[]);
    assert!(!text.contains("costs\n"), "{text}");
}
