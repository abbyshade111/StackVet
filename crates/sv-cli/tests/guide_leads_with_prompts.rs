//! Backlog 0512: the guide's first three steps lead with letting the AI tool do them through the setup prompts, and
//! the typed steps are the fallback: the prompts are offered before step 1, as the easiest way, and step 1 opens by
//! sending someone the prompts already set up on to step 4, so they do not do it all again by hand.

use std::path::Path;

fn guide() -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/GETTING-STARTED.md"),
    )
    .unwrap()
}

#[test]
fn the_prompts_come_first_and_the_typed_steps_are_the_fallback() {
    let guide = guide();
    let offer = guide
        .find("**Easiest: let your AI coding tool do steps 1 to 3.** [The setup prompts](prompts/setup.md)")
        .expect("the guide offers the setup prompts as the easiest way");
    let step_one = guide.find("\n## 1. ").expect("the guide has a step 1");
    assert!(
        offer < step_one,
        "the prompts are offered after step 1 begins"
    );
    let offer_paragraph = &guide[offer..offer + guide[offer..].find("\n\n").unwrap()];
    assert!(
        offer_paragraph.contains("below do it by hand"),
        "the offer says the steps below are the way by hand: {offer_paragraph}"
    );
}

#[test]
fn step_one_sends_someone_the_prompts_set_up_on_to_step_four() {
    let guide = guide();
    let step_one = guide.find("\n## 1. ").expect("the guide has a step 1");
    let after_heading = &guide[step_one + 1..];
    let first_paragraph = after_heading
        .split("\n\n")
        .nth(1)
        .expect("step 1 has a first paragraph");
    assert!(
        first_paragraph.contains("setup prompts") && first_paragraph.contains("Go to step 4"),
        "{first_paragraph}"
    );
    assert!(guide.contains("\n## 4. "), "there is a step 4 to go to");
}
