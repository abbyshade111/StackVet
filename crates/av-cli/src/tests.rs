//! What `av report` takes on its command line.

use super::*;

fn words(args: &[&str]) -> Vec<String> {
    args.iter().map(|a| (*a).to_owned()).collect()
}

#[test]
fn a_folder_and_the_two_options_are_read_in_any_order() {
    assert_eq!(
        parse(&words(&["--json", "app", "--hook-record"])),
        Ok(Options {
            folder: PathBuf::from("app"),
            hook_record: true,
            json: true,
        })
    );
    // The hook record is never read unless asked for.
    assert!(!parse(&words(&["app"])).unwrap().hook_record);
}

#[test]
fn no_folder_a_second_folder_or_an_unknown_option_is_refused() {
    assert!(parse(&words(&[])).is_err());
    assert!(parse(&words(&["a", "b"])).is_err());
    assert!(parse(&words(&["a", "--transcript"])).is_err());
}
