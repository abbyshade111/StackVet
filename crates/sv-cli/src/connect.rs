//! `sv connect TOOL`: the settings block that connects an AI coding tool to `sv`, printed with the
//! paths already right (backlog 0217, part 1).
//!
//! Setting up by hand meant typing the app folder's full path into three places of a settings file,
//! and a wrong one fails without a word: the tool simply has no `stackvet_` tools. This prints the
//! block instead, for Claude (`.mcp.json`), VS Code (`.vscode/mcp.json`), or Cursor
//! (`.cursor/mcp.json`), with the folder it was started in. It prints and never writes: the AI coding
//! tool, or the person, puts it in the file, as with every other file in the app's folder.
//!
//! With no tool named, it prints StackVet's server itself, in no tool's format: its name, that it talks over standard
//! input and output, and its command and arguments (backlog 0396). The setup prompts use that one, and the AI coding
//! tool puts it into its own app's settings in its own app's way, so `sv` keeps no list of apps for them: the owner's
//! decision of 13 October 2026, after the trial in Antigravity stopped at an app `sv connect` had no settings for.
//!
//! Two ways to run `sv`, two blocks:
//! - `--docker PATH`: through the container, as `docs/GETTING-STARTED.md` sets it up. `PATH` is where
//!   `docker` is on the person's computer (`which docker` prints it). A program inside the container
//!   cannot see that, so it is never guessed; the folder it can see, because the container is started
//!   with the folder mounted at its own path (`-v "$PWD":"$PWD" -w "$PWD"`).
//! - without it: this copy of `sv`, installed on the computer, by its real place. Inside the container
//!   that place means nothing outside it, so there it stops and asks for `--docker`.
//!
//! With `--docker`, `--keys DIR` names the person's key folder on their computer (`~/.config/stackvet`), and the
//! container is given its list of trusted keys, `allowed_signers`, one file, read-only, and nothing else of the
//! folder (backlog 0581, the owner's choice of 15 October 2026): the folder also holds the key that signs as them.
//! Without the list, the container could not count what they recorded with `sv review`. The file must be there before
//! the container starts, or Docker makes a folder in its place; the setup makes it empty first.
//!
//! # What it is worth
//!
//! Nothing, as evidence: it reads no file of the app's and credits nothing. A block that is right is
//! shown to work only when the tool lists the `stackvet_` tools afterwards, which the guide and the
//! setup prompt both ask for.

use anyhow::{Context, Result, bail};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use sv_frameworks::paths::Canonical;

/// The AI coding tools `sv connect` knows the settings file of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Claude,
    VsCode,
    Cursor,
}

impl Tool {
    pub const NAMES: &'static str = "claude, vscode, or cursor";

    pub fn named(name: &str) -> Option<Tool> {
        match name {
            "claude" => Some(Tool::Claude),
            "vscode" => Some(Tool::VsCode),
            "cursor" => Some(Tool::Cursor),
            _ => None,
        }
    }

    /// Where the block goes, from the app's folder.
    pub fn settings_file(self) -> &'static str {
        match self {
            Tool::Claude => ".mcp.json",
            Tool::VsCode => ".vscode/mcp.json",
            Tool::Cursor => ".cursor/mcp.json",
        }
    }
}

/// How the tool starts `sv`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Runner {
    /// Through the container: `docker` at this path on the person's computer, and, on Linux, the
    /// person's own user and group, so the files it writes are theirs.
    Docker {
        docker: String,
        user: Option<String>,
        /// The person's key folder on their computer, whose list of trusted keys the container reads (backlog 0581).
        keys: Option<String>,
    },
    /// This copy of `sv`, at its real place.
    Installed(PathBuf),
}

/// The settings block for `tool`, starting `sv` for `folder`.
pub fn block(tool: Tool, runner: &Runner, folder: &Path) -> Value {
    let (command, args) = command_and_args(runner, folder);
    match tool {
        Tool::Claude | Tool::Cursor => json!({
            "mcpServers": { "stackvet": { "command": command, "args": args } }
        }),
        Tool::VsCode => json!({
            "servers": { "stackvet": { "type": "stdio", "command": command, "args": args } }
        }),
    }
}

/// StackVet's server for `folder`, in no tool's format (backlog 0396): what any AI coding tool needs to put it into
/// its own settings.
pub fn server(runner: &Runner, folder: &Path) -> Value {
    let (command, args) = command_and_args(runner, folder);
    json!({ "name": SERVER_NAME, "transport": "stdio", "command": command, "args": args })
}

/// The name every block gives the server, and so the prefix of its tools' names (`stackvet_status`).
pub const SERVER_NAME: &str = "stackvet";

/// The program that starts `sv` as an MCP server for `folder`, and its arguments.
fn command_and_args(runner: &Runner, folder: &Path) -> (String, Vec<String>) {
    command_and_args_on(runner, folder, cfg!(windows))
}

/// What the container is told the app's folder is on the person's computer, when that is a Windows path (backlog 0375,
/// ADR-153): the MCP server writes the paths it hands back under it, and reads the ones it is given from it.
pub const HOST_FOLDER_ENV: &str = "SV_HOST_FOLDER";

/// A Windows folder as a Linux container shows it: the drive letter in lower case, then the rest with `/` for `\`
/// (`C:\Users\you\app` is `/c/Users/you/app`). `None` for a path that does not start at a drive's root.
pub fn inside_path(folder: &str) -> Option<String> {
    if !is_drive_path(folder) {
        return None;
    }
    let drive = folder[..1].to_ascii_lowercase();
    let rest = folder[2..].replace('\\', "/");
    let rest = rest.trim_end_matches('/');
    Some(format!("/{drive}{rest}"))
}

/// `command_and_args`, with whether this is Windows given, so the Windows settings are tested everywhere.
fn command_and_args_on(runner: &Runner, folder: &Path, windows: bool) -> (String, Vec<String>) {
    let folder = folder.display().to_string();
    // On Windows the folder is shown inside the container at a Linux-style path, and the container is told the
    // Windows one; anywhere else, at its own path, as before.
    let inside = match runner {
        Runner::Docker { .. } if windows => inside_path(&folder),
        _ => None,
    };
    let (command, args) = match runner {
        Runner::Docker { docker, user, keys } => {
            let mut args = vec![
                "run".to_owned(),
                "-i".to_owned(),
                "--rm".to_owned(),
                // The MCP server needs no network, and is given none (design entry 0043, "sv in a container").
                "--network".to_owned(),
                "none".to_owned(),
                "-v".to_owned(),
                format!("{folder}:{}", inside.as_deref().unwrap_or(&folder)),
            ];
            if inside.is_some() {
                args.push("-e".to_owned());
                args.push(format!("{HOST_FOLDER_ENV}={folder}"));
            }
            if let Some(keys) = keys {
                // The list alone, read-only, at the place `sv` looks for it once XDG_CONFIG_HOME names the folder above.
                args.push("-v".to_owned());
                args.push(format!(
                    "{keys}/{list}:{CONTAINER_CONFIG}/{dir}/{list}:ro",
                    list = sv_check::signed::TRUSTED_FILE,
                    dir = sv_frameworks::names::CONFIG_DIR,
                ));
                args.push("-e".to_owned());
                args.push(format!("XDG_CONFIG_HOME={CONTAINER_CONFIG}"));
            }
            if let Some(user) = user {
                args.push("--user".to_owned());
                args.push(user.clone());
            }
            args.push(sv_frameworks::names::IMAGE.to_owned());
            (docker.clone(), args)
        }
        Runner::Installed(program) => (program.display().to_string(), Vec::new()),
    };
    let mut args = args;
    args.extend([
        "mcp".to_owned(),
        "--root".to_owned(),
        inside.unwrap_or(folder),
    ]);
    (command, args)
}

/// Where the container's settings folder is, when it is given the list of trusted keys.
const CONTAINER_CONFIG: &str = "/sv-config";

/// The hooks block that keeps the hook record for the app in `folder` (backlog 0566, ADR-122): Claude Code's, for the
/// person to add to its settings. Each runs `program hook-record --app folder` after a command or file write, whether it
/// finished or failed, and at the end of each turn.
pub fn hook_block(program: &Path, folder: &Path) -> Value {
    let command = format!(
        "{} hook-record --app {}",
        shell_word(&program.display().to_string()),
        shell_word(&folder.display().to_string())
    );
    let hooks = json!([{ "type": "command", "command": command }]);
    let matcher = format!("^({})$", crate::hook_record::TOOLS.join("|"));
    json!({
        "hooks": {
            "PostToolUse": [{ "matcher": matcher, "hooks": hooks }],
            "PostToolUseFailure": [{ "matcher": matcher, "hooks": hooks }],
            "Stop": [{ "hooks": hooks }],
        }
    })
}

/// A path as one word of a shell command: in single quotes, a quote inside it closed, escaped, and opened again.
fn shell_word(word: &str) -> String {
    format!("'{}'", word.replace('\'', r"'\''"))
}

/// `sv connect [TOOL] [--docker PATH] [--user UID:GID] [--keys DIR] [--folder DIR] [--hook]`.
pub fn command(args: &[String]) -> Result<String> {
    let mut tool = None;
    let mut hook = false;
    let mut docker = None;
    let mut user = None;
    let mut keys = None;
    let mut folder = None;
    let mut words = args.iter();
    while let Some(arg) = words.next() {
        match arg.as_str() {
            "--docker" => docker = Some(words.next().context("--docker needs docker's full path")?),
            "--user" => user = Some(words.next().context("--user needs UID:GID")?),
            "--keys" => {
                keys =
                    Some(words.next().context(
                        "--keys needs your key folder, such as \"$HOME/.config/stackvet\"",
                    )?)
            }
            "--folder" => {
                folder = Some(PathBuf::from(
                    words.next().context("--folder needs a folder")?,
                ))
            }
            "--hook" => hook = true,
            other if other.starts_with("--") => bail!("unknown option: {other}"),
            other => {
                tool = Some(Tool::named(other).with_context(|| {
                    format!(
                        "`{other}` is not a tool `sv connect` knows: give {}",
                        Tool::NAMES
                    )
                })?)
            }
        }
    }
    let folder = folder.unwrap_or_else(|| PathBuf::from("."));
    let folder = folder
        .canonical()
        .with_context(|| format!("{} is not a folder `sv` can open", folder.display()))?;
    if !folder.is_dir() {
        bail!("{} is not a folder", folder.display());
    }
    if hook {
        return hook_for(tool, docker.is_some(), &folder);
    }
    let runner = match docker {
        Some(docker) => {
            check_docker_on(docker, cfg!(windows))?;
            if let Some(user) = user {
                check_user(user)?;
            }
            let keys = keys.map(|k| check_keys(k)).transpose()?;
            Runner::Docker {
                docker: docker.clone(),
                user: user.cloned(),
                keys,
            }
        }
        None => {
            if user.is_some() {
                bail!("--user is for the container: give it with --docker");
            }
            if keys.is_some() {
                bail!(
                    "--keys is for the container: give it with --docker. A copy of `sv` installed on your computer \
                     reads your key folder itself"
                );
            }
            if in_a_container() {
                bail!(
                    "this `sv` is running inside its container, so its own path means nothing to your \
                     AI coding tool. Give --docker with docker's full path on your computer (`which \
                     docker` prints it), for example: sv connect {}--docker /usr/local/bin/docker",
                    tool.map(|t| format!("{} ", tool_word(t)))
                        .unwrap_or_default()
                );
            }
            let program = std::env::current_exe()
                .and_then(|p| p.canonical())
                .context("could not find where this copy of `sv` is")?;
            Runner::Installed(program)
        }
    };
    let printed = match tool {
        Some(tool) => block(tool, &runner, &folder),
        None => server(&runner, &folder),
    };
    let text = serde_json::to_string_pretty(&printed)?;
    Ok(format!("{text}\n"))
}

/// `sv connect claude --hook`: the hook block, for an `sv` installed on this computer, which the hook runs.
fn hook_for(tool: Option<Tool>, docker: bool, folder: &Path) -> Result<String> {
    if tool != Some(Tool::Claude) {
        bail!(
            "--hook prints Claude Code's hook, the one tool whose hook input is documented for it: give `claude`. Other \
             AI coding tools are not offered one yet"
        );
    }
    if docker || in_a_container() {
        bail!(
            "--hook is not offered for sv in its container yet: the hook runs sv after each of the tool's commands, and \
             needs a copy of sv installed on your computer (docs/INSTALL.md)"
        );
    }
    let program = std::env::current_exe()
        .and_then(|p| p.canonical())
        .context("could not find where this copy of `sv` is")?;
    let text = serde_json::to_string_pretty(&hook_block(&program, folder))?;
    Ok(format!("{text}\n"))
}

/// The line that says where the block goes, for standard error, so standard output is the block alone.
pub fn where_it_goes(args: &[String]) -> String {
    if args.iter().any(|a| a == "--hook") {
        return "Add this to .claude/settings.local.json in the app's folder, under \"hooks\" with any hooks already \
                there, then restart Claude Code. It runs sv after each command Claude Code runs and each file it \
                writes, and records what it did outside this folder (no file contents, output, or prompts). Nothing \
                reads that record unless you run `sv report --hook-record`."
            .to_owned();
    }
    match args.iter().find_map(|a| Tool::named(a)) {
        Some(tool) => format!(
            "Put this in {} in the app's folder, then restart your AI coding tool and ask it to list the \
             `stackvet_` tools. {OLDER}",
            tool.settings_file()
        ),
        None => "This is StackVet's MCP server for this folder, in no app's format: a local server that talks over \
                 standard input and output. Put its name, command, and arguments, exactly as printed, into your AI \
                 coding tool's MCP settings, in that tool's own way; then have it load them and list the `stackvet_` \
                 tools. "
            .to_owned()
            + OLDER,
    }
}

/// What `sv connect` adds for a tool that keeps one MCP setting for the whole account (backlog 0541): a server
/// started for an earlier app can still be running, and the tool talking to it.
pub const OLDER: &str = "Each app's folder has a StackVet server of its own. If your AI tool keeps one MCP setting for your \
     whole account, a server started for an earlier app may still be running: stop it (`docker ps` lists it, `docker \
     stop` stops it) or restart the tool, then check that `stackvet_status` names this folder.";

fn tool_word(tool: Tool) -> &'static str {
    match tool {
        Tool::Claude => "claude",
        Tool::VsCode => "vscode",
        Tool::Cursor => "cursor",
    }
}

/// `UID:GID`, two numbers, as `id -u` and `id -g` print them.
fn check_user(user: &str) -> Result<()> {
    let numbers = user.split_once(':').filter(|(u, g)| {
        !u.is_empty()
            && !g.is_empty()
            && u.bytes().all(|b| b.is_ascii_digit())
            && g.bytes().all(|b| b.is_ascii_digit())
    });
    if numbers.is_none() {
        bail!(
            "--user needs UID:GID, two numbers, such as 1000:1000 (`id -u` and `id -g` print yours)"
        );
    }
    Ok(())
}

/// The key folder as Docker is given it: a full path, without a trailing `/`, and holding no `:` or `,`, which Docker
/// reads as separators. Outside the container its list of trusted keys must be there already: Docker would make a folder
/// in its place. Inside, the person's folders cannot be seen, and the setup makes the file first.
fn check_keys(keys: &str) -> Result<String> {
    check_keys_on(keys, cfg!(windows))
}

/// `check_keys`, with whether this is Windows given, so the drive's colon is tested everywhere.
fn check_keys_on(keys: &str, windows: bool) -> Result<String> {
    let trimmed = keys.trim_end_matches(['/', '\\']);
    // On Windows a full path starts with its drive (`C:\Users\…`), whose colon Docker Desktop reads as part of the
    // path; past it, a colon is a separator there as everywhere.
    let drive = windows && !trimmed.starts_with('/') && is_drive_path(trimmed);
    if !trimmed.starts_with('/') && !drive {
        bail!(
            "--keys needs your key folder's full path, such as \"$HOME/.config/stackvet\" (the shell fills in \
             $HOME), not {keys}"
        );
    }
    let past_drive = if drive {
        trimmed.get(2..).unwrap_or_default()
    } else {
        trimmed
    };
    if past_drive.contains([':', ',']) {
        bail!(
            "--keys cannot name a folder whose path holds `:` or `,`, which Docker reads as separators: {keys}"
        );
    }
    let list = Path::new(trimmed).join(sv_check::signed::TRUSTED_FILE);
    if !in_a_container() && !list.is_file() {
        bail!(
            "{} is not there yet, and Docker would make a folder in its place. Make it first: mkdir -p {trimmed} && \
             chmod 700 {trimmed} && touch {}",
            list.display(),
            list.display()
        );
    }
    Ok(trimmed.to_owned())
}

/// Docker's path as the settings give it: a full one, since an app started from the Dock (or the Start menu) often
/// cannot find a bare `docker`. On Windows that is a path from a drive's root (`C:\Program Files\Docker\...\docker.exe`;
/// backlog 0375, ADR-153), and anywhere it may start with `/`.
fn check_docker_on(docker: &str, windows: bool) -> Result<()> {
    if docker.starts_with('/') || (windows && is_drive_path(docker)) {
        return Ok(());
    }
    if windows {
        bail!(
            "--docker needs docker's full path, such as \"C:\\Program Files\\Docker\\Docker\\resources\\bin\\docker.exe\" \
             (`where.exe docker` prints it), because an app started from the Start menu often cannot find a bare \
             `docker`"
        );
    }
    bail!(
        "--docker needs docker's full path, such as /usr/local/bin/docker (`which docker` prints it), because an \
         app started from the Dock often cannot find a bare `docker`"
    );
}

/// A Windows path from a drive's root, `C:\…` or `C:/…`: read by hand, so it is told on any computer.
fn is_drive_path(path: &str) -> bool {
    let b = path.as_bytes();
    b.len() >= 3 && b[0].is_ascii_alphabetic() && b[1] == b':' && (b[2] == b'\\' || b[2] == b'/')
}

/// Docker marks every container with `/.dockerenv`.
pub fn in_a_container() -> bool {
    Path::new("/.dockerenv").exists() || std::env::var_os("SV_CONNECT_IN_CONTAINER").is_some()
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod windows_docker_tests;

#[cfg(test)]
mod windows_keys_tests;
