//! Backlog 0499: `locate.log-destinations` points the notes question on the log inventory (V16.1.1), the hand check on
//! where logs go (V16.2.3), and the design question on sending logs to a separate system (V16.4.3) at where the code
//! sets its logs up, keeps them, or sends them. Held to what it must find and to the look-alikes it must leave alone.

use super::*;
use crate::test_support::scratch;

fn shipped() -> Locators {
    Locators::load(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/locators.json"),
    )
    .expect("the shipped locators load, every query compiled")
}

/// The places one locator found in a folder holding one file, as `file:line`.
fn found(locator: &str, name: &str, text: &str) -> Vec<String> {
    let dir = scratch("");
    std::fs::write(dir.join(name), text).unwrap();
    let listing = sv_scan::files::Listing::of(&dir);
    // The setup worked: the file was listed as the app's own code, or finding nothing would mean nothing.
    assert_eq!(listing.app_files().count(), 1, "{name} was not listed");
    shipped()
        .find(&listing)
        .places
        .get(locator)
        .map(|places| {
            places
                .iter()
                .map(|p| format!("{}:{}", p.file, p.line))
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn python_log_destinations_are_found_and_log_calls_are_not() {
    let places = found(
        "locate.log-destinations",
        "logs.py",
        "logging.basicConfig(level=logging.INFO)\n\
         h = RotatingFileHandler('app.log', maxBytes=10000)\n\
         s = logging.handlers.SysLogHandler(address='/dev/log')\n\
         sentry_sdk.init(dsn=DSN)\n\
         cw = watchtower.CloudWatchLogHandler()\n\
         log = logging.getLogger(__name__)\n\
         log.info('started')\n\
         print('hello')\n",
    );
    assert_eq!(
        places,
        [
            "logs.py:1",
            "logs.py:2",
            "logs.py:3",
            "logs.py:4",
            "logs.py:5"
        ]
    );
}

#[test]
fn javascript_log_destinations_are_found_and_log_calls_are_not() {
    let places = found(
        "locate.log-destinations",
        "logger.js",
        "const t = new winston.transports.File({ filename: 'app.log' });\n\
         const p = pino.transport({ target: 'pino-pretty' });\n\
         Sentry.init({ dsn });\n\
         const l = new Logtail(token);\n\
         logger.info('started');\n\
         console.log('hello');\n\
         const c = new winston.transports.Console();\n",
    );
    assert_eq!(
        places,
        ["logger.js:1", "logger.js:2", "logger.js:3", "logger.js:4"]
    );
}
