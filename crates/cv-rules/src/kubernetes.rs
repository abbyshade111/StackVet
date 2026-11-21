//! Kubernetes manifests, read with `sv`'s YAML reader into one [`Unit`] per container of each workload: a `Pod`, or the
//! pod template of a `Deployment`, `StatefulSet`, `DaemonSet`, `ReplicaSet`, `Job`, or `CronJob`. Init containers are
//! left out, since they finish before the app starts.
//!
//! The user a container runs as is its own `securityContext`'s, or else the pod's, as Kubernetes applies them.

use crate::unit::{EnvVar, Layer, Unit};
use sv_yaml::{Value, parse_documents};

/// Whether a YAML file's text looks like a Kubernetes manifest, before it is parsed.
pub fn looks_like(text: &str) -> bool {
    text.contains("apiVersion:") && text.contains("kind:")
}

/// The containers of the file's workloads, or why it could not be read. A document of another kind (a `Service`, a
/// `ConfigMap`) holds none.
pub fn read(relative: &str, text: &str) -> Result<Vec<Unit>, String> {
    let mut out = Vec::new();
    for document in parse_documents(text)? {
        let Some(kind) = document.get("kind").and_then(Value::text) else {
            continue;
        };
        let spec = document.get("spec");
        let pod = match kind {
            "Pod" => spec,
            "Deployment" | "StatefulSet" | "DaemonSet" | "ReplicaSet" | "Job" => spec
                .and_then(|s| s.get("template"))
                .and_then(|t| t.get("spec")),
            "CronJob" => spec
                .and_then(|s| s.get("jobTemplate"))
                .and_then(|j| j.get("spec"))
                .and_then(|s| s.get("template"))
                .and_then(|t| t.get("spec")),
            _ => None,
        };
        let Some(pod) = pod else { continue };
        let workload = document
            .get("metadata")
            .and_then(|m| m.get("name"))
            .and_then(Value::text)
            .unwrap_or("unnamed");
        let pod_context = pod.get("securityContext");
        let Some(Value::Seq(containers)) = pod.get("containers") else {
            continue;
        };
        for container in containers {
            let name = container
                .get("name")
                .and_then(Value::text)
                .unwrap_or("unnamed");
            let mut unit = Unit::new(
                Layer::Kubernetes,
                relative,
                container.line().unwrap_or(1),
                format!("container `{name}` of {kind} `{workload}`"),
            );
            unit.image = container
                .get("image")
                .and_then(Value::text)
                .map(str::to_owned);
            unit.env = env(container.get("env"));
            unit.env_elsewhere = container.get("envFrom").map(|_| "envFrom".to_owned());
            let own = container.get("securityContext");
            unit.user = field(own, "runAsUser").or_else(|| field(pod_context, "runAsUser"));
            unit.run_as_non_root =
                field(own, "runAsNonRoot").or_else(|| field(pod_context, "runAsNonRoot"));
            out.push(unit);
        }
    }
    Ok(out)
}

fn field(context: Option<&Value>, name: &str) -> Option<(String, usize)> {
    match context?.get(name)? {
        Value::Scalar(s, line) => Some((s.clone(), *line)),
        _ => None,
    }
}

/// `env: [{name, value}]`; an entry with `valueFrom` has a value the file does not hold.
fn env(value: Option<&Value>) -> Vec<EnvVar> {
    let Some(Value::Seq(items)) = value else {
        return Vec::new();
    };
    items
        .iter()
        .filter_map(|item| {
            let name = item.get("name").and_then(Value::text)?;
            Some(EnvVar {
                name: name.to_owned(),
                value: item.get("value").and_then(Value::text).map(str::to_owned),
                line: item.line().unwrap_or(1),
            })
        })
        .collect()
}
