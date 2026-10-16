//! Starting the containers beside the app: the sidecar, the mail server, the browser and its driver, the name
//! server, the test sign-in provider, the stand-ins for switched-off services, and the test model. Moved out of
//! `docker.rs` unchanged (backlog 0324).

use super::*;

impl DockerBackend {
    /// Starts the container every request to the app is sent from, on the app's fenced network.
    ///
    /// It has nothing to be allowed and one thing to write, so it is given a read-only file system
    /// with one small folder in memory for the request it is about to send (`PROBE_TMPFS`), no
    /// capabilities, and no way to gain privileges. It runs `sleep` and nothing else until
    /// a request is `exec`ed into it. `--rm` and the time limit mean a run that dies without its
    /// teardown still leaves nothing behind for long.
    pub(super) fn start_sidecar(&self, network: &str, name: &str, plan: &RunPlan) -> bool {
        let limit = sidecar_seconds(plan).to_string();
        matches!(
            self.docker(&[
                "run",
                "-d",
                "--rm",
                "--name",
                name,
                "--network",
                network,
                "--tmpfs",
                PROBE_TMPFS,
                PROBE_IMAGE,
                "sleep",
                &limit,
            ]),
            Ok((0, _))
        )
    }

    /// Starts the mail server on the app's fenced network.
    ///
    /// Hardened as the sidecar is, but for one place to write: Mailpit keeps its messages in a file,
    /// and that file goes in memory rather than anywhere on this computer. It accepts any user name
    /// and password over plain SMTP, so an app written to sign in to its mail server is not refused
    /// by this one; there is nothing behind it to protect.
    pub(super) fn start_mail(&self, network: &str, name: &str) -> bool {
        matches!(self.docker(&mail_args(network, name)), Ok((0, _)))
    }

    pub(super) fn start_browser(&self, network: &str, name: &str) -> bool {
        matches!(self.docker(&browser_args(network, name)), Ok((0, _)))
    }

    /// Starts the stand-in name server on the app's fenced network (ADR-085). Its script is passed
    /// on the command line, as the test provider's is, so nothing is written to the owner's disk.
    pub(super) fn start_name_server(&self, network: &str, name: &str) -> bool {
        let args = crate::name_server::start_args(network, name);
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        matches!(self.docker(&args), Ok((0, _)))
    }

    /// The address Docker gave `name` on `network`, read from Docker's own record. `None` when it
    /// has none, so the app is not pointed at an address nothing answers from.
    pub(super) fn address_on(&self, name: &str, network: &str) -> Option<String> {
        let template = format!("{{{{(index .NetworkSettings.Networks \"{network}\").IPAddress}}}}");
        let (code, out) = self.docker(&["inspect", "-f", &template, name]).ok()?;
        let address = out.trim();
        (code == 0 && !address.is_empty()).then(|| address.to_owned())
    }

    /// Carries the browser's `localhost:<port>` to the app. Refused for the two ports Chromium's
    /// own DevTools use, where the forwarder would take the place of the thing it is driving.
    pub(super) fn forward_browser(&self, name: &str, app: &str, port: u16) -> bool {
        if port == 9222 || port == 9223 {
            return false;
        }
        let listen = format!("TCP4-LISTEN:{port},fork,reuseaddr,bind=127.0.0.1");
        let to = format!("TCP4:{app}:{port}");
        matches!(
            self.docker(&["exec", "-d", name, "socat", &listen, &to]),
            Ok((0, _))
        )
    }

    pub(super) fn start_provider(&self, network: &str, name: &str, secret: &str) -> bool {
        let (args, secrets) = provider_start(network, name, secret);
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        matches!(self.docker_with_secrets(&args, &secrets), Ok((0, _)))
    }

    /// Starts a copy of the app under another name with one more setting in its environment: the
    /// same image, folder, network, and settings as the first, so the only difference between the
    /// two answers is the setting.
    pub(super) fn start_switched_off(
        &self,
        app_args: &[String],
        app: &str,
        name: &str,
        image: &str,
        setting: &str,
    ) -> bool {
        let Some(args) = switched_off_args(app_args, app, name, image, setting) else {
            return false;
        };
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        matches!(self.docker(&args), Ok((0, _)))
    }

    pub(super) fn start_model(&self, network: &str, name: &str) -> bool {
        let host = format!("HOST={name}");
        let port = format!("PORT={MODEL_PORT}");
        matches!(
            self.docker(&helper_args(network, name, &[&host, &port], MODEL_SCRIPT)),
            Ok((0, _))
        )
    }
}
