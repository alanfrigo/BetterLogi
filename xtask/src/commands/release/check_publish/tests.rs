use serde_json::{Value, json};

use super::validate;

fn package(name: &str, publish: &Value, dependencies: &Value) -> Value {
    json!({
        "name": name,
        "publish": publish,
        "dependencies": dependencies,
    })
}

fn dependency(name: &str, req: &str, kind: &Value) -> Value {
    json!({
        "name": name,
        "req": req,
        "path": format!("/workspace/{name}"),
        "kind": kind,
    })
}

#[test]
fn rejects_unpublished_path_dependency_of_published_package() {
    let metadata = json!({
        "packages": [
            package(
                "betterlogi-cli",
                &Value::Null,
                &json!([dependency("betterlogi-ipc", "^0.7.7", &Value::Null)]),
            ),
            package("betterlogi-ipc", &json!([]), &json!([])),
        ],
    });

    let error = validate(&metadata.to_string()).unwrap_err().to_string();

    assert!(
        error.contains("`betterlogi-cli` depends on unpublished path package `betterlogi-ipc`")
    );
}

#[test]
fn rejects_path_dependency_without_registry_version() {
    let metadata = json!({
        "packages": [
            package(
                "betterlogi-ipc",
                &Value::Null,
                &json!([dependency("betterlogi-core", "*", &Value::Null)]),
            ),
            package("betterlogi-core", &Value::Null, &json!([])),
        ],
    });

    let error = validate(&metadata.to_string()).unwrap_err().to_string();

    assert!(error.contains(
        "`betterlogi-ipc` path dependency on `betterlogi-core` must declare a registry version"
    ));
}

#[test]
fn accepts_publishable_normal_and_build_closure_and_ignores_dev_dependencies() {
    let metadata = json!({
        "packages": [
            package(
                "betterlogi-cli",
                &Value::Null,
                &json!([
                    dependency("betterlogi-core", "^0.7.8", &Value::Null),
                    dependency("betterlogi-build", "^0.7.8", &json!("build")),
                    dependency("betterlogi-test", "*", &json!("dev")),
                ]),
            ),
            package("betterlogi-core", &Value::Null, &json!([])),
            package("betterlogi-build", &Value::Null, &json!([])),
            package("betterlogi-test", &json!([]), &json!([])),
        ],
    });

    assert_eq!(validate(&metadata.to_string()).unwrap(), 3);
}
