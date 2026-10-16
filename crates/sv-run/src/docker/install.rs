//! The install step, before the run and outside the fence: the image it uses and what its failure says (ADR-052).
//! Moved out of `docker.rs` unchanged (backlog 0324).

use super::*;

impl DockerBackend {
    /// Fills `install`'s volume, unless an earlier run already filled it from the same files in the
    /// same image (ADR-052). Whether it was reused, or why it could not be done.
    pub(super) fn install(
        &self,
        install: &crate::install::Install,
        name: &str,
        image: &str,
    ) -> Result<bool, CannotRun> {
        let backend = |e: String| CannotRun::BackendFailed { detail: e };
        // A volume counts as filled only when the install that filled it finished: it writes its
        // mark last. One an interrupted install left half full is emptied and filled again.
        let mark = format!("/v/{}", crate::install::INSTALLED_MARK);
        let look = format!("{}:/v:ro", install.volume);
        let finished = self
            .docker(&["volume", "inspect", &install.volume])
            .is_ok_and(|(code, _)| code == 0)
            && self
                .docker(&[
                    "run",
                    "--rm",
                    "--network",
                    "none",
                    "-v",
                    &look,
                    PROBE_IMAGE,
                    "test",
                    "-f",
                    &mark,
                ])
                .is_ok_and(|(code, _)| code == 0);
        if finished {
            return Ok(true);
        }
        let _ = self.docker(&["volume", "rm", "-f", &install.volume]);
        let label = format!(
            "{}={}",
            crate::install::VOLUME_LABEL,
            install.ecosystem.short()
        );
        let (code, out) = self
            .docker(&["volume", "create", "--label", &label, &install.volume])
            .map_err(backend)?;
        if code != 0 {
            return Err(backend(first_line(&out)));
        }
        let args = install.args(name, image);
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        let (code, out) = self.docker(&refs).map_err(backend)?;
        if code != 0 {
            let _ = self.docker(&["volume", "rm", "-f", &install.volume]);
            return Err(CannotRun::InstallFailed {
                registry: install.ecosystem.registry(),
                detail: install_failure(&out),
            });
        }
        Ok(false)
    }

    /// The image's own `PATH`, so the installed packages' commands can go in front of it. The
    /// usual one when the image does not say.
    pub(super) fn image_path(&self, image: &str) -> String {
        self.docker(&[
            "image",
            "inspect",
            "--format",
            "{{json .Config.Env}}",
            image,
        ])
        .ok()
        .filter(|(code, _)| *code == 0)
        .and_then(|(_, out)| serde_json::from_str::<Vec<String>>(out.trim()).ok())
        .and_then(|env| {
            env.into_iter()
                .find_map(|pair| pair.strip_prefix("PATH=").map(str::to_owned))
        })
        .unwrap_or_else(|| {
            "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin".to_owned()
        })
    }
}

/// What an install that did not finish said, shortly: its last few lines that are not blank.
pub(super) fn install_failure(out: &str) -> String {
    let lines: Vec<&str> = out
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    let tail = lines[lines.len().saturating_sub(3)..].join(" / ");
    let tail: String = tail.chars().take(400).collect();
    if tail.is_empty() {
        "it said nothing.".to_owned()
    } else {
        format!("it ended with: {tail}.")
    }
}
