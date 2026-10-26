//! Every command `sv` knows, the options each takes, and its lines of `sv --help`, in the order the help lists them.
//! Moved out of `main.rs` on 21 October 2026 (backlog 0324), word for word; `main.rs` reads it as before.

/// One command: its name, the options it takes, and its lines of the help.
pub(crate) struct Command {
    pub(crate) name: &'static str,
    /// What the one word that is not an option names, such as `PATH`, or `None` when it takes none.
    pub(crate) word: Option<&'static str>,
    /// Options that stand alone.
    pub(crate) flags: &'static [&'static str],
    /// Options followed by a value, the word after them.
    pub(crate) valued: &'static [&'static str],
    /// The command's lines of `sv --help`, as they are printed there.
    pub(crate) help: &'static str,
}

/// Every command `sv` knows, in the order `sv --help` lists them.
pub(crate) const COMMANDS: &[Command] = &[
    Command {
        name: "init",
        word: None,
        flags: &[],
        valued: &[],
        help: "  sv init            print the stackvet.toml spec to hand to your AI coding tool\n",
    },
    Command {
        name: "scope",
        word: Some("PATH"),
        flags: &[],
        valued: &[],
        help: "  sv scope [PATH]    show which requirements apply to the app, and why\n",
    },
    Command {
        name: "plan",
        word: Some("PATH"),
        flags: &[],
        valued: &[],
        help: "  sv plan [PATH]     before any code: what applies, what to decide, the tests to write,\n                     and what the app must give `sv run`; credits nothing\n",
    },
    Command {
        name: "preflight",
        word: Some("PATH"),
        flags: &[],
        valued: &[],
        help: "  sv preflight [PATH]\n                     once there is code: whether it gives `sv run` what stackvet.toml\n                     says, read from the files and never run; credits nothing\n",
    },
    Command {
        name: "brief",
        word: Some("PATH"),
        flags: &[],
        valued: &["--feature"],
        help: "  sv brief [PATH] --feature FEATURE
                     before building one feature (sign-in, uploads, payments, ai, ...):
                     its requirements, what to decide, the rules to code by, the tests to
                     write, and what `sv run` needs; credits nothing. No --feature lists them
",
    },
    Command {
        name: "notes",
        word: Some("PATH"),
        flags: &[],
        valued: &[],
        help: "  sv notes [PATH]    write security-notes.md: the questions only you can answer\n",
    },
    Command {
        name: "questions",
        word: Some("PATH"),
        flags: &[],
        valued: &[],
        help: "  sv questions [PATH]\n                     the questions only a person can answer, for your AI coding\n                     tool to ask you: paste them into its chat\n",
    },
    Command {
        name: "rules",
        word: Some("PATH"),
        flags: &["--print"],
        valued: &[],
        help: "  sv rules [PATH] [--print]\n                     write the security rules your AI coding tool follows while it\n                     codes into AGENTS.md (--print shows them instead)\n",
    },
    Command {
        name: "explain",
        word: Some("ID"),
        flags: &[],
        valued: &["--app", "--report"],
        help: "  sv explain ID [--app DIR | --report FILE]\n                     one requirement, such as V7.4.1: what it asks, the checks that speak\n                     to it and the kind of run each needs, and what to do; --app adds what\n                     the app's last report said about it, --report what that report said\n",
    },
    Command {
        name: "prompts",
        word: Some("report"),
        flags: &[],
        valued: &["--requirement", "--app", "--report"],
        help: "  sv prompts [report | --requirement ID | --app DIR | --report FILE]\n                     prompts to give your AI coding tool, each saying whether it has\n                     been shown to work; --requirement gives only those for one requirement\n                     or Secure by Design control, such as V1.2.4 or SBD-AC-03; --app gives\n                     those for what the app's last report (DIR/stackvet-report/report.json,\n                     or --report FILE) shows unproven; `report` gives the one that has your\n                     AI coding tool read the report with you, what was not checked first\n",
    },
    Command {
        name: "probe",
        word: Some("URL"),
        flags: &[],
        valued: &["--hsts-preload", "--api"],
        help: "  sv probe URL [--hsts-preload FILE] [--api PATH]\n                     ask your own live site the few things only it can answer;\n                     --api names an address of the app's API, such as /api/health,\n                     to ask over plain HTTP the way a program asks (one more request)\n",
    },
    Command {
        name: "run",
        word: Some("PATH"),
        flags: &["--slow"],
        valued: &[],
        help: "  sv run [PATH] [--slow]\n                     start the app behind the network fence, check it answers, ask it\n                     questions as a stranger and as the test users, and run its tests;\n                     with `install = true` it first downloads the packages the app names,\n                     in a container that sees only the dependency files;\n                     --slow also waits out the session timeouts you state,\n                     and ten minutes before using an emailed sign-in code\n                     exit status: 0 the app ran; 2 not assessed: it could not be started\n                     or never answered; 3 sv itself failed (no stackvet.toml, a bad manifest)\n",
    },
    Command {
        name: "check",
        word: Some("PATH"),
        flags: &[],
        valued: &["--fail-on", "--baseline"],
        help: "  sv check [PATH] [--fail-on WHAT] [--baseline DIR]\n                     credentials left in the code, what the rules that read the code find,\n                     and how it is set up: a narrower scan than `sv report` (or\n                     stackvet_check), saying nothing about requirements; it reads\n                     stackvet.toml when it is there only to stop on one it cannot read\n                     --fail-on also fails for attention[:SEVERITY] (a finding at SEVERITY\n                     or worse: critical, high, medium, low (the default), or info),\n                     not-assessed (also a symbolic link not followed, or an entry that is not an\n                     ordinary file), or any (both), several separated by commas\n                     --baseline DIR, an older report folder: attention fails only for a\n                     finding that report did not hold; every finding is still listed\n                     exit status: 0 finished; 1 needs attention (only with --fail-on);\n                     2 not assessed: a check could not run, or no file of the app was read;\n                     3 sv itself failed (no such folder, an option it does not know, a\n                     stackvet.toml it cannot read)\n",
    },
    Command {
        name: "sbom",
        word: Some("PATH"),
        flags: &[],
        valued: &[],
        help: "  sv sbom [PATH]     print the list of what the app ships, as CycloneDX JSON\n",
    },
    Command {
        name: "audit",
        word: Some("PATH"),
        flags: &[],
        valued: &["--advisories"],
        help: "  sv audit [PATH] [--advisories DIR]\n                     match what the app ships against a local OSV database (DIR, or the\n                     folder SV_ADVISORY_DIR names)\n                     exit status: 0 everything compared and nothing matched; 1 a known\n                     vulnerability; 2 the comparison did not cover the whole app (no\n                     database, an ecosystem it lacks, a list of packages not complete);\n                     3 sv itself failed (an unreadable database or manifest, no such folder)\n",
    },
    Command {
        name: "report",
        word: Some("PATH"),
        flags: &[
            "--run",
            "--slow",
            "--tools",
            "--no-tools",
            "--keep-tool-output",
            "--quiet",
            "--hook-record",
        ],
        valued: &[
            "--out",
            "--advisories",
            "--fail-on",
            "--baseline",
            "--codeql-results",
        ],
        help: "  sv report [PATH | ADDRESS] [--out DIR] [--run [--slow]] [--no-tools | --keep-tool-output] [--advisories DIR] [--codeql-results FILE] [--fail-on WHAT] [--baseline DIR] [--quiet] [--hook-record]\n                     write the reports: what applies, what was found, what nobody has answered,\n                     into PATH/stackvet-report unless --out says where; --run starts the app\n                     as `sv run` does, downloading its packages first with `install = true`\n                     ADDRESS, a public project's https://github.com/OWNER/PROJECT or\n                     https://gitlab.com/GROUP/PROJECT address, in place of PATH: its code is\n                     downloaded once, as an archive, into stackvet-fetched in this folder,\n                     and checked without running any of it (not yet with --run)\n                     the outside tools (Semgrep, Bandit, and the rest) run unless\n                     --no-tools; Semgrep fetches its rules from semgrep.dev as it runs;\n                     CodeQL runs only with [tools] codeql = true in stackvet.toml\n                     --keep-tool-output keeps each tool's own report,\n                     credentials cut out, in seen.json beside the report\n                     --quiet leaves out the progress and timing lines; counts and errors stay\n                     --hook-record reads, for this report only, the record kept by the\n                     hook `sv connect claude --hook` prints; it credits nothing\n                     --codeql-results FILE, CodeQL's SARIF from your own CI: read only when it\n                     is of the commit checked out, with nothing changed since; any other\n                     file is named in the report and credits nothing\n                     --fail-on also fails for attention[:SEVERITY] (a finding at SEVERITY\n                     or worse: critical, high, medium, low (the default), or info),\n                     not-assessed (also a symbolic link not followed, an outside tool that\n                     could not run, or an --advisories comparison that did not cover the app),\n                     or any (both), several separated by commas\n                     --baseline DIR, an older report folder: attention fails only for a\n                     finding that report did not hold; every finding is still listed and\n                     counted, and those it held are marked\n                     exit status: 0 finished; 1 needs attention (only with --fail-on);\n                     2 not assessed: a check could not run, no file of the app was read,\n                     or --run was given and the app could not be started;\n                     3 sv itself failed (no stackvet.toml, a bad manifest, no such folder)\n",
    },
    Command {
        name: "review",
        word: Some("PATH"),
        flags: &["--no-report"],
        valued: &[],
        help: "  sv review [PATH] [--no-report]\n                     record, in your own terminal, the findings you set aside, the answers\n                     you confirm, and your own answers and checks made by hand; only what\n                     you record here counts as yours\n                     when something was recorded, the report is written again with the\n                     options of the last one (--run included), unless --no-report\n",
    },
    Command {
        name: "before-live",
        word: Some("PATH"),
        flags: &[],
        valued: &[],
        help: "  sv before-live [PATH]\n                     print again the checklist before going live kept with the app's last\n                     report: what to do or confirm, in order; it credits nothing, checks\n                     nothing, and writes nothing\n",
    },
    Command {
        name: "bundle",
        word: Some("PATH"),
        flags: &["--run", "--slow", "--tools", "--no-tools"],
        valued: &["--out", "--advisories"],
        help: "  sv bundle [PATH] [--out FILE.zip] [--run [--slow]] [--no-tools] [--advisories DIR]\n                     the app, its report and a SHA-256 for every file in one zip, beside\n                     the app unless --out says where, with anything that could hold a\n                     secret left out and listed\n                     the outside tools (Semgrep, Bandit, and the rest) run unless\n                     --no-tools; Semgrep fetches its rules from semgrep.dev as it runs;\n                     CodeQL runs only with [tools] codeql = true in stackvet.toml\n",
    },
    Command {
        name: "dashboard",
        word: Some("FOLDER..."),
        flags: &[],
        valued: &["--out"],
        help: "  sv dashboard FOLDER... --out FILE.html\n                     one page for several apps, from the report already in each one's\n                     stackvet-report folder: every app in alphabetical order, and each\n                     app's own view; it checks nothing itself, and writes only FILE.html\n",
    },
    Command {
        name: "compare",
        word: Some("REPORT..."),
        flags: &[],
        valued: &[],
        help: "  sv compare OLDER [NEWER]\n                     what changed between two reports of an app: each requirement whose\n                     status moved, with what it gained or lost, and the findings that\n                     came or went; NEWER is this folder's report when not given; writes\n                     nothing\n",
    },
    Command {
        name: "history",
        word: Some("ACTION..."),
        flags: &["--all"],
        valued: &[],
        help: "  sv history on|off|status|forget FOLDER|forget --all\n                     keep a small record of each sv report run, for sv dashboard to show\n                     how an app changes; off until you turn it on, kept outside every\n                     app's folder, readable only by you, and never your code\n",
    },
    Command {
        name: "connect",
        word: Some("TOOL"),
        flags: &["--hook"],
        valued: &["--docker", "--user", "--keys", "--folder"],
        help: "  sv connect [TOOL] [--docker PATH] [--user UID:GID] [--keys DIR] [--folder DIR] [--hook]\n                     print StackVet's MCP server for this folder, the paths already filled\n                     in: with no TOOL, its name, command, and arguments, for any AI coding\n                     tool to put in its own settings; with TOOL (claude, vscode, or\n                     cursor), that tool's settings block;\n                     --docker PATH (where docker is, from `which docker`) for the\n                     container, --user UID:GID on Linux, --keys DIR (your key folder,\n                     \"$HOME/.config/stackvet\") so the container can count what you\n                     recorded in sv review: it reads that folder's list of trusted keys\n                     alone, read-only; writes nothing\n                     claude --hook prints instead a Claude Code hook that records each\n                     command the tool runs and each file it writes, outside the app's\n                     folder, for sv report --hook-record to read\n",
    },
    Command {
        name: "hook-record",
        word: None,
        flags: &[],
        valued: &["--app"],
        help: "  sv hook-record --app DIR\n                     what the hook sv connect claude --hook prints runs: adds one line\n                     to DIR's hook record for what Claude Code hands it; not for typing\n",
    },
    Command {
        name: "doctor",
        word: Some("PATH"),
        flags: &[],
        valued: &[],
        help: "  sv doctor [PATH]   is everything ready? which sv this is, whether the folder is in\n                     git, whether stackvet.toml reads and says how to start the app, and\n                     whether Docker can start it; opens no network connection, writes nothing\n",
    },
    Command {
        name: "mcp",
        word: None,
        flags: &[],
        valued: &["--root", "--time-limit"],
        help: "  sv mcp [--root DIR] [--time-limit SECONDS]\n                     serve the checks to an AI coding tool over MCP, for the apps under DIR\n                     (the current folder when none is given);\n                     a check that takes longer than SECONDS (50) is reported as not finished\n",
    },
];
