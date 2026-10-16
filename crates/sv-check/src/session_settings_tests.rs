//! Backlog 0612: the session lifetime the code sets, read and compared with the policy's.

use super::*;

fn read(key: &str, file: &str, source: &str) -> Vec<(usize, String, u64)> {
    in_source(key, file, source)
        .into_iter()
        .map(|s| (s.line, s.name, s.seconds))
        .collect()
}

#[test]
fn django_and_flask_lifetimes_are_read_in_seconds() {
    let found = read(
        "python",
        "settings.py",
        "SESSION_COOKIE_AGE = 60 * 60 * 24 * 7\n\
         PERMANENT_SESSION_LIFETIME = timedelta(hours=8)\n\
         app.config['PERMANENT_SESSION_LIFETIME'] = datetime.timedelta(days=1, hours=2)\n\
         app.permanent_session_lifetime = 1800\n\
         CACHE_TIMEOUT = 600\n\
         SESSION_COOKIE_AGE = env('AGE')\n\
         response.set_cookie('csrf', token, max_age=31536000)\n",
    );
    assert_eq!(
        found,
        [
            (1, "SESSION_COOKIE_AGE".to_owned(), 604_800),
            (2, "PERMANENT_SESSION_LIFETIME".to_owned(), 28_800),
            (3, "PERMANENT_SESSION_LIFETIME".to_owned(), 93_600),
            (4, "permanent_session_lifetime".to_owned(), 1_800),
        ]
    );
}

#[test]
fn an_express_session_s_max_age_is_read_in_milliseconds_and_a_cache_s_is_not() {
    for key in ["javascript", "typescript"] {
        let found = read(
            key,
            "server.js",
            "app.use(session({ secret: s, cookie: { httpOnly: true, maxAge: 7 * 24 * 60 * 60 * 1000 } }));\n\
             app.use(cookieSession({ name: 'session', maxAge: 3600000 }));\n\
             app.use(express.static('public', { maxAge: 86400000 }));\n\
             res.cookie('pref', v, { maxAge: 31536000000 });\n\
             const t = jwt.sign(p, k, { expiresIn: '30d' });\n",
        );
        assert_eq!(
            found,
            [
                (1, "maxAge".to_owned(), 604_800),
                (2, "maxAge".to_owned(), 3_600),
            ],
            "{key}"
        );
    }
}

#[test]
fn a_lifetime_longer_than_the_policy_is_a_finding_and_a_shorter_one_is_not() {
    let settings = [
        Setting {
            file: "settings.py".into(),
            line: 1,
            name: "SESSION_COOKIE_AGE".into(),
            seconds: 604_800,
        },
        Setting {
            file: "settings.py".into(),
            line: 2,
            name: "PERMANENT_SESSION_LIFETIME".into(),
            seconds: 1_800,
        },
    ];
    let found = longer_than_policy(&settings, 480);
    assert_eq!(found.len(), 1);
    let f = &found[0];
    assert_eq!(f.rule_id, DIFFERS);
    assert_eq!(f.requirement_ids, ["V7.1.1"]);
    assert_eq!(
        (f.location.file.as_str(), f.location.line),
        ("settings.py", 1)
    );
    assert!(f.description.contains("sets 7 days"), "{}", f.description);
    assert!(
        f.description.contains("8 hours at most"),
        "{}",
        f.description
    );
    // Equal is not longer.
    assert!(longer_than_policy(&settings[1..], 30).is_empty());
}

#[test]
fn durations_are_said_in_the_largest_whole_unit() {
    assert_eq!(in_words(604_800), "7 days");
    assert_eq!(in_words(86_400), "1 day");
    assert_eq!(in_words(28_800), "8 hours");
    assert_eq!(in_words(2_700), "45 minutes");
    assert_eq!(in_words(90), "90 seconds");
    assert_eq!(duration("7d"), Some(604_800));
    assert_eq!(duration("2 days"), Some(172_800));
    assert_eq!(duration("soon"), None);
}
