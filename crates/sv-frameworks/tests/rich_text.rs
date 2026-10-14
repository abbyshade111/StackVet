//! V1.3.1 (rich text from an editor) and V1.3.5 (Markdown, stylesheets, templates, and the like that people supply)
//! are placed by the app's own answer to `rich-text`, or by an AI feature, whose answers are text the app did not
//! write either (backlog 0505, the owner's choice of 14 October 2026; ADR-117). Unanswered, they are not assessed,
//! never ruled out: before this, an app with no AI had both ruled out whatever it took from people.

use std::path::PathBuf;
use sv_frameworks::Frameworks;
use sv_frameworks::applicability::{ApplicabilityConfig, Buckets, ConditionContext, bucket};
use sv_frameworks::{Condition, Source};

const BOTH: [&str; 2] = ["V1.3.1", "V1.3.5"];

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data")
}

/// Every question the owner is asked answered no, and then `set` on top, placed at level 2, where both live.
fn placed(set: &[(Condition, Option<bool>)]) -> Buckets {
    let config = ApplicabilityConfig::load_v2(
        &data_dir().join("knowledge"),
        &data_dir().join("applicability-v2.json"),
    )
    .unwrap();
    let f = Frameworks::load(&data_dir().join("frameworks")).unwrap();
    let mut ctx = ConditionContext::default();
    // A condition given as `None` is left unanswered: never set at all.
    let unanswered = |c: &Condition| set.iter().any(|(s, v)| s == c && v.is_none());
    for c in Condition::ALL
        .iter()
        .filter(|c| c.source() == Source::Claim && !unanswered(c))
    {
        ctx.set(*c, false);
    }
    for (c, v) in set {
        if let Some(v) = v {
            ctx.set(*c, *v);
        }
    }
    bucket(&f, &config, &ctx, 2)
}

fn applies(b: &Buckets, id: &str) -> bool {
    b.applicable.iter().any(|a| a == id)
}

#[test]
fn with_no_rich_text_and_no_ai_both_are_ruled_out_saying_both() {
    let b = placed(&[]);
    for id in BOTH {
        let na = b
            .not_applicable
            .iter()
            .find(|na| na.id == id)
            .unwrap_or_else(|| panic!("{id} with every no"));
        assert!(na.reason.contains("no AI feature"), "{id}: {}", na.reason);
    }
}

#[test]
fn rich_text_alone_or_an_ai_feature_alone_makes_both_apply() {
    for set in [
        [
            (Condition::RichText, Some(true)),
            (Condition::Ai, Some(false)),
        ],
        [
            (Condition::RichText, Some(false)),
            (Condition::Ai, Some(true)),
        ],
    ] {
        let b = placed(&set);
        for id in BOTH {
            assert!(applies(&b, id), "{id} with {set:?}");
        }
    }
}

#[test]
fn rich_text_unanswered_and_no_ai_leaves_both_not_assessed_on_that_question() {
    let b = placed(&[(Condition::RichText, None)]);
    for id in BOTH {
        let open = b
            .not_assessed
            .iter()
            .find(|na| na.id == id)
            .unwrap_or_else(|| panic!("{id} was placed with rich-text unanswered"));
        assert_eq!(open.blocked_on, [Condition::RichText], "{id}");
    }
    // With an AI feature, the open question no longer matters: it applies either way.
    let b = placed(&[(Condition::RichText, None), (Condition::Ai, Some(true))]);
    for id in BOTH {
        assert!(applies(&b, id), "{id}");
    }
}
