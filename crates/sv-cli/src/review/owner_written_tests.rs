//! Before `sv review` asks whether an answer given as the owner's is theirs, it says the AI coding tool can write
//! `by = "owner"`, and a `how`, too (backlog 0593): in the Cursor run of 15 October 2026 the tool wrote answers and
//! checks by hand in the owner's name, and the review showed them as if they were the owner's, and the owner sealed them.

use super::tests::{HEAD, Scratch, with_app};

#[test]
fn an_answer_given_as_the_owners_is_said_to_be_maybe_the_tools_before_it_is_asked_about() {
    let s = Scratch::new("owner-written");
    with_app(
        &s,
        &format!(
            "{HEAD}[design]\n\"V8.3.1\" = {{ answer = \"yes\", where = \"app.py\", by = \"owner\" }}\n\n\
             [checked-by-hand.'V12.2.2']\nresult = \"done\"\non = \"2026-10-01\"\nby = \"owner\"\n\
             how = \"Opened the live site; the padlock shows a trusted certificate.\"\n"
        ),
    );
    // Enter for both: left as they are.
    let (result, out) = s.run("\n\n");
    result.unwrap();
    assert!(
        out.contains("It is given as yours"),
        "the setup: both offered: {out}"
    );
    let said = "Your AI coding tool can write `by = \"owner\"` as well as you can, so this answer, and any `how` \
                beside it, may be the tool's words, not yours. Record it only if you gave the answer, or made the \
                check, yourself.";
    assert_eq!(
        out.split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .matches(said)
            .count(),
        2,
        "said before each of the two: {out}"
    );
    // Said before the question, not after it.
    let warned = out.find("may be the tool's words").unwrap();
    let asked = out.find("Type `owner` if this is your own answer").unwrap();
    assert!(warned < asked, "{out}");
}
