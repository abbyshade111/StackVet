//! A Terraform plan as JSON (`terraform show -json plan.out`), read for the containers it will run: today, the
//! container definitions of each `aws_ecs_task_definition` in `planned_values`, child modules included.
//!
//! A plan is read rather than the `.tf` files because it holds the values after Terraform has worked out the variables
//! and modules, which reading `.tf` text cannot. A container definitions value that is still unknown at plan time
//! (shown in `after_unknown`, absent from `planned_values`) is simply not there, and nothing is said about it.

use crate::unit::{EnvVar, Layer, Unit};
use serde_json::Value;

/// Whether a JSON file's text looks like a plan, before it is parsed.
pub fn looks_like(text: &str) -> bool {
    text.contains("\"planned_values\"") && text.contains("\"format_version\"")
}

/// The ECS containers the plan will run, or why it could not be read.
pub fn read(relative: &str, text: &str) -> Result<Vec<Unit>, String> {
    let plan: Value = serde_json::from_str(text).map_err(|e| format!("it is not JSON: {e}"))?;
    let Some(root) = plan
        .get("planned_values")
        .and_then(|p| p.get("root_module"))
    else {
        return Err("it has no `planned_values.root_module`".to_owned());
    };
    let mut resources = Vec::new();
    collect(root, &mut resources);
    let mut out = Vec::new();
    for resource in resources {
        if resource.get("type").and_then(Value::as_str) != Some("aws_ecs_task_definition") {
            continue;
        }
        let address = resource
            .get("address")
            .and_then(Value::as_str)
            .unwrap_or("aws_ecs_task_definition");
        let line = line_of(text, address);
        let Some(definitions) = resource
            .get("values")
            .and_then(|v| v.get("container_definitions"))
            .and_then(Value::as_str)
        else {
            continue;
        };
        let containers: Value = serde_json::from_str(definitions)
            .map_err(|e| format!("the container definitions of `{address}` are not JSON: {e}"))?;
        for container in containers.as_array().into_iter().flatten() {
            let name = container
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or("unnamed");
            let mut unit = Unit::new(
                Layer::TerraformPlan,
                relative,
                line,
                format!("ECS container `{name}` of `{address}`"),
            );
            unit.image = container
                .get("image")
                .and_then(Value::as_str)
                .map(str::to_owned);
            for variable in container
                .get("environment")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                if let Some(name) = variable.get("name").and_then(Value::as_str) {
                    unit.env.push(EnvVar {
                        name: name.to_owned(),
                        value: variable
                            .get("value")
                            .and_then(Value::as_str)
                            .map(str::to_owned),
                        line,
                    });
                }
            }
            // A secret's value is held by Secrets Manager or Parameter Store, never in the plan.
            for secret in container
                .get("secrets")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                if let Some(name) = secret.get("name").and_then(Value::as_str) {
                    unit.env.push(EnvVar {
                        name: name.to_owned(),
                        value: None,
                        line,
                    });
                }
            }
            if container
                .get("environmentFiles")
                .and_then(Value::as_array)
                .is_some_and(|files| !files.is_empty())
            {
                unit.env_elsewhere = Some("environmentFiles".to_owned());
            }
            unit.user = container
                .get("user")
                .and_then(Value::as_str)
                .map(|u| (u.to_owned(), line));
            out.push(unit);
        }
    }
    Ok(out)
}

fn collect<'a>(module: &'a Value, out: &mut Vec<&'a Value>) {
    out.extend(
        module
            .get("resources")
            .and_then(Value::as_array)
            .into_iter()
            .flatten(),
    );
    for child in module
        .get("child_modules")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        collect(child, out);
    }
}

/// The line the resource's address is first written on, for a finding to point at; the first line when it is not
/// found as written.
fn line_of(text: &str, address: &str) -> usize {
    let needle = format!("\"address\": \"{address}\"");
    let compact = format!("\"address\":\"{address}\"");
    text.lines()
        .position(|l| l.contains(&needle) || l.contains(&compact))
        .map_or(1, |i| i + 1)
}
