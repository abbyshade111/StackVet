//! The WebSocket after sign-out is judged only on a sign-out that was done (backlog 0560, ADR-119): one refused
//! on a 200 page ends nothing, and the socket opening afterwards would be blamed on the app.

use super::super::fake_app::*;
use super::super::tests::ws_run;
use super::*;

#[test]
fn a_sign_out_refused_on_a_200_page_is_not_a_websocket_that_outlived_it() {
    let o = ws_run(Flaws {
        logout_refused_on_200: true,
        ..Default::default()
    });
    assert!(
        !rule_ids(&o).contains(&WS_AFTER_SIGN_OUT.rule_id),
        "a refused sign-out was read as one the WebSocket outlived: {:?}",
        o.findings
    );
    assert!(
        o.not_assessed
            .iter()
            .any(|(id, why)| id == "V4.4.3"
                && why.contains("the sign-out was answered 200 with a page")),
        "{:?}",
        o.not_assessed
    );
}

#[test]
fn a_websocket_that_outlives_a_sign_out_that_was_done_is_still_found() {
    let o = ws_run(Flaws {
        ws_survives_sign_out: true,
        ..Default::default()
    });
    assert!(
        rule_ids(&o).contains(&WS_AFTER_SIGN_OUT.rule_id),
        "{:?}",
        o.not_assessed
    );
}
