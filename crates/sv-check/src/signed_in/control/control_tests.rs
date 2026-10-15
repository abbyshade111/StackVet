use super::*;

fn answer(status: u16, location: Option<&str>, body: &str) -> ProbeResponse {
    ProbeResponse {
        id: "t".to_owned(),
        status,
        headers: location
            .map(|l| vec![("location".to_owned(), l.to_owned())])
            .unwrap_or_default(),
        body: body.to_owned(),
    }
}

#[test]
fn a_form_shown_again_with_200_where_the_control_redirected_cannot_be_told() {
    // Backlog 0523: Flask's way of refusing a form.
    let control = answer(302, Some("/login"), "");
    let refused = answer(
        200,
        None,
        "<form><input name=email> Enter a valid email address.</form>",
    );
    assert_eq!(
        beside(&control, &Some(refused)),
        Beside::Unclear("200 where the control redirected to /login".to_owned())
    );
}

#[test]
fn the_same_redirect_is_answered_like_the_control() {
    let control = answer(303, Some("http://app:8080/login?next=1"), "");
    let same = answer(302, Some("/login"), "");
    assert_eq!(beside(&control, &Some(same)), Beside::Like);
}

#[test]
fn a_redirect_elsewhere_cannot_be_told() {
    let control = answer(303, Some("/login"), "");
    let back = answer(303, Some("/signup?error=email"), "");
    assert_eq!(
        beside(&control, &Some(back)),
        Beside::Unclear(
            "sent the browser to /signup where the control was sent to /login".to_owned()
        )
    );
}

#[test]
fn a_refusal_is_a_refusal_and_a_crash_or_silence_is_not() {
    let control = answer(303, Some("/login"), "");
    assert_eq!(
        beside(&control, &Some(answer(422, None, ""))),
        Beside::Refused
    );
    assert!(matches!(
        beside(&control, &Some(answer(500, None, ""))),
        Beside::Unclear(why) if why.contains("500")
    ));
    assert!(matches!(beside(&control, &None), Beside::Unclear(_)));
}

#[test]
fn two_successes_of_another_shape_cannot_be_told() {
    let control = answer(200, None, r#"{"id": 4, "email": "a@example.com"}"#);
    let error = answer(200, None, r#"{"error": "email too long"}"#);
    assert_eq!(
        beside(&control, &Some(error)),
        Beside::Unclear(
            "an answer with the names `error` where the control's had `email`, `id`".to_owned()
        )
    );
    let same = answer(200, None, r#"{"email": "b@example.com", "id": 5}"#);
    assert_eq!(beside(&control, &Some(same)), Beside::Like);

    let page = answer(200, None, "<h1>Welcome</h1>");
    let form = answer(200, None, "<form method=post><input name=email></form>");
    assert!(matches!(
        beside(&page, &Some(form.clone())),
        Beside::Unclear(_)
    ));
    // A form beside a form is the same shape: this reads no words.
    assert_eq!(beside(&form, &Some(form.clone())), Beside::Like);
}
