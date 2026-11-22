use super::*;

#[test]
fn curl_is_held_to_https_no_redirects_the_limits_and_headers_from_its_input() {
    let args = curl_args_for_test();
    let after = |flag: &str| {
        let at = args
            .iter()
            .position(|a| a == flag)
            .unwrap_or_else(|| panic!("{flag} is missing: {args:?}"));
        args[at + 1].clone()
    };
    assert_eq!(
        args[0], "--disable",
        "--disable must come first, or curl reads a .curlrc"
    );
    assert_eq!(after("--proto"), "=https");
    assert_eq!(after("--max-redirs"), "0");
    assert_eq!(after("--max-filesize"), MOST_ANSWER_BYTES.to_string());
    assert_eq!(after("--max-time"), MOST_SECONDS.to_string());
    // The headers, the token's among them, come from standard input, never the command line.
    assert_eq!(after("--header"), "@-");
    assert!(
        !args
            .iter()
            .any(|a| a.contains("Authorization") || a.contains("Bearer")),
        "{args:?}"
    );
    assert!(
        !args
            .iter()
            .any(|a| a == "--insecure" || a == "-k" || a == "--location" || a == "-L"),
        "{args:?}"
    );
}

#[test]
fn the_last_answers_headers_are_read() {
    let dumped = "HTTP/2 301\r\nlocation: https://api.github.com/x\r\n\r\nHTTP/2 200\r\nX-OAuth-Scopes: repo, gist\r\n\
                  x-accepted-github-permissions: metadata=read\r\n\r\n";
    let headers = parse_headers(dumped);
    let answer = Answer {
        status: 200,
        headers,
        ..Answer::default()
    };
    assert_eq!(answer.header("X-OAuth-Scopes"), Some("repo, gist"));
    assert_eq!(
        answer.header("x-accepted-github-permissions"),
        Some("metadata=read")
    );
    assert_eq!(
        answer.header("location"),
        None,
        "the redirect's headers are not this answer's"
    );
}
