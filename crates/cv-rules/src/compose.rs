//! A compose file (`compose.yaml`, `docker-compose.yml`, and their kin), read with `sv`'s YAML reader into one
//! [`Unit`] per service: its `image`, `build`, `environment`, `env_file`, and `user`.

use crate::unit::{Build, EnvVar, Layer, Unit, dir_of, join};
use sv_yaml::{Value, parse};

/// Whether a file's name says it is a compose file.
pub fn is_compose(file_name: &str) -> bool {
    let lower = file_name.to_ascii_lowercase();
    let stem = lower
        .strip_suffix(".yml")
        .or_else(|| lower.strip_suffix(".yaml"));
    stem.is_some_and(|s| s.starts_with("docker-compose") || s.starts_with("compose"))
}

/// Each service of the file, or why it could not be read.
pub fn read(relative: &str, text: &str) -> Result<Vec<Unit>, String> {
    let document = parse(text)?;
    let Some(Value::Map(services)) = document.get("services") else {
        return Err("it has no `services`".to_owned());
    };
    let dir = dir_of(relative);
    let mut out = Vec::new();
    for (name, service) in services {
        let line = service.line().unwrap_or(1);
        let mut unit = Unit::new(
            Layer::Compose,
            relative,
            line,
            format!("compose service `{name}`"),
        );
        unit.image = service
            .get("image")
            .and_then(Value::text)
            .map(str::to_owned);
        unit.build = build(service, dir);
        unit.env = environment(service.get("environment"));
        unit.env_elsewhere = service.get("env_file").map(|v| match v {
            Value::Scalar(s, _) => s.clone(),
            Value::Seq(items) => items
                .iter()
                .find_map(|i| i.text().or_else(|| i.get("path").and_then(Value::text)))
                .unwrap_or("env_file")
                .to_owned(),
            _ => "env_file".to_owned(),
        });
        unit.user = service.get("user").and_then(|v| match v {
            Value::Scalar(s, l) => Some((s.clone(), *l)),
            _ => None,
        });
        out.push(unit);
    }
    Ok(out)
}

/// `build: ./api` or `build: { context: ., dockerfile: docker/Dockerfile.prod }`: the Dockerfile, relative to the
/// folder. A context that is a web address or leaves the folder is not followed.
fn build(service: &Value, dir: &str) -> Option<Build> {
    let value = service.get("build")?;
    let line = value.line().unwrap_or(1);
    let (context, dockerfile) = match value {
        Value::Scalar(context, _) => (context.as_str(), "Dockerfile"),
        Value::Map(_) => (
            value.get("context").and_then(Value::text).unwrap_or("."),
            value
                .get("dockerfile")
                .and_then(Value::text)
                .unwrap_or("Dockerfile"),
        ),
        _ => return None,
    };
    if context.contains("://") || context.starts_with("git@") {
        return None;
    }
    let context = join(dir, context)?;
    Some(Build {
        dockerfile: join(&context, dockerfile)?,
        line,
    })
}

/// `environment:` as a map (`NODE_ENV: production`) or a list (`- NODE_ENV=production`, or `- NODE_ENV` alone, which
/// passes the shell's value through).
fn environment(value: Option<&Value>) -> Vec<EnvVar> {
    match value {
        Some(Value::Map(pairs)) => pairs
            .iter()
            .map(|(name, v)| EnvVar {
                name: name.clone(),
                value: v.text().map(str::to_owned),
                line: v.line().unwrap_or(1),
            })
            .collect(),
        Some(Value::Seq(items)) => items
            .iter()
            .filter_map(|item| match item {
                Value::Scalar(s, line) => Some(match s.split_once('=') {
                    Some((name, value)) => EnvVar {
                        name: name.to_owned(),
                        value: Some(value.to_owned()),
                        line: *line,
                    },
                    None => EnvVar {
                        name: s.clone(),
                        value: None,
                        line: *line,
                    },
                }),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}
