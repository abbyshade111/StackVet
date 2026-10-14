//! An app that gives its server an empty host listens on every address, as one naming `0.0.0.0` does (backlog 0399).
//! In the owner's trial the AI coding tool changed `HOST`'s default from `"0.0.0.0"` to `""` after Bandit flagged the
//! first; the app listened as before, and the preflight then said no address was found.

use super::*;

fn listen(app: &str) -> (Answer, String) {
    let manifest = Manifest::parse(
        "manifest-version = 1\n[app]\nname = \"t\"\n[stack.run]\nimage = \"python:3.12-slim\"\n\
         start = \"python server.py\"\nhealth = \"/\"\n",
        Path::new("stackvet.toml"),
    )
    .expect("the manifest parses");
    let source = Source {
        files: [("server.py".to_owned(), app.to_owned())].into(),
        unread: Vec::new(),
    };
    let items = preflight(&manifest, &source);
    let item = items
        .iter()
        .find(|i| i.topic == "listen")
        .expect("a listen item");
    (item.answer, item.text(&Fence::none()))
}

#[test]
fn each_empty_host_reads_as_every_address() {
    for app in [
        "import os\nHOST = os.environ.get(\"HOST\", \"\")\nserve((HOST, 8080))\n",
        "import os\nHOST = os.environ.get('HOST', '')\nserve((HOST, 8080))\n",
        "HOST = \"\"\nserve((HOST, 8080))\n",
        "HOST = ''\nserve((HOST, 8080))\n",
        "from http.server import HTTPServer\nHTTPServer((\"\", 8080), Handler).serve_forever()\n",
        "import socket\ns = socket.socket()\ns.bind(('', 8080))\n",
        "s.bind((\"::\", 8080))\n",
    ] {
        let (answer, said) = listen(app);
        assert_eq!(answer, Answer::Looks, "{app}: {said}");
        assert!(
            said.to_lowercase().contains("every address"),
            "{app}: {said}"
        );
    }
}

#[test]
fn an_empty_string_elsewhere_is_not_an_address() {
    // The setup: the same app with `0.0.0.0` reads as every address.
    assert_eq!(listen("serve(('0.0.0.0', 8080))\n").0, Answer::Looks);
    for app in [
        "NAME = \"\"\nserve(('127.0.0.1', 8080))\n",
        "print(\"\", 8080)\nserve(('127.0.0.1', 8080))\n",
        "x = os.environ.get(\"PATH\", \"\")\nserve(('127.0.0.1', 8080))\n",
    ] {
        assert_eq!(listen(app).0, Answer::Look, "{app}: loopback only");
    }
}
