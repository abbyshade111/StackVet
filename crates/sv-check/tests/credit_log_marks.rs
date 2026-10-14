//! A credit's mark is written to the census when it changes (backlog 0430): the line `Verified::new`
//! writes is made before `.in_part()` is chained on, so it cannot carry the mark. This test is a binary
//! of its own, so the one variable it sets is read by nothing else.

/// Marked in part once (a second mark changes nothing and writes nothing), then made whole again: two
/// lines of four after the credit's own, each with the place that changed it.
#[test]
fn a_credit_marked_in_part_and_made_whole_writes_each_change_once() {
    let dir = std::env::temp_dir().join(format!("sv-credit-log-marks-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mine = dir.join("credits.log");
    let ci = std::env::var_os("SV_CREDIT_LOG");
    // SAFETY: the only test in this binary, and no thread of its own has started.
    unsafe { std::env::set_var("SV_CREDIT_LOG", &mine) };
    let made = line!() + 2;
    let credit =
        sv_check::Verified::new("probe.password-hints", &["V6.4.2"], "a test's own".into());
    let marked = line!() + 1;
    let mut credit = credit.in_part().in_part_if(true);
    assert!(credit.in_part);
    let whole = line!() + 1;
    credit.whole_after_all();
    assert!(!credit.in_part);
    credit.whole_after_all();
    let written = std::fs::read_to_string(&mine).unwrap();
    let file = file!();
    assert_eq!(
        written,
        format!(
            "probe.password-hints\tV6.4.2\t{file}:{made}\n\
             probe.password-hints\tV6.4.2\tin part\t{file}:{marked}\n\
             probe.password-hints\tV6.4.2\twhole\t{file}:{whole}\n"
        )
    );
    if let Some(ci) = ci {
        unsafe { std::env::set_var("SV_CREDIT_LOG", ci) };
    }
    std::fs::remove_dir_all(&dir).unwrap();
}
