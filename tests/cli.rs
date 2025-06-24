use assert_cmd::Command;
use std::path::PathBuf;

fn store_path(test_name: &str) -> PathBuf {
    PathBuf::from(format!("./tests/{}/projects.json", test_name))
}

fn cmd_with_store(test_name: &str) -> Command {
    let store = store_path(test_name);
    let mut cmd = Command::cargo_bin("rpj").unwrap();
    cmd.env("RPJ_STORE_PATH", store);
    cmd
}

fn cleanup_store(test_name: &str) {
    let store_dir = format!("./tests/{}", test_name);
    let export_file = format!("./{}.rpj", test_name);

    if std::path::Path::new(&store_dir).exists() {
        std::fs::remove_dir_all(&store_dir).unwrap();
    }

    if std::path::Path::new(&export_file).exists() {
        std::fs::remove_file(&export_file).unwrap();
    }
}

fn new_project(name: &str) {
    let dir = format!("./tests/{}", name);
    std::fs::create_dir_all(&dir).unwrap();

    cmd_with_store(name)
        .arg("new")
        .args([
            name,
            &dir,
            "--run-cmd",
            "echo 'Hello, World!'",
            "--description",
            "Test project",
        ])
        .assert()
        .success();
}

fn remove_project(name: &str) {
    cmd_with_store(name)
        .arg("remove")
        .arg(name)
        .assert()
        .success();
}

#[test]
fn test_help_command() {
    Command::cargo_bin("rpj")
        .unwrap()
        .arg("--help")
        .assert()
        .failure();
}

#[test]
fn test_new_update_remove() {
    let test_name = "new_update_remove";
    new_project(test_name);

    // Update project
    cmd_with_store(test_name)
        .arg("update")
        .args([test_name, "--description", "Updated description"])
        .assert()
        .success();

    remove_project(test_name);
    cleanup_store(test_name);
}

#[test]
fn test_export_and_add() {
    let test_name = "export_add";
    new_project(test_name);

    // Export project
    cmd_with_store(test_name)
        .arg("export")
        .args([test_name])
        .assert()
        .success();

    remove_project(test_name);

    // Add project from export
    cmd_with_store(test_name)
        .arg("add")
        .args(["./export_add.rpj", "--delete-after"])
        .assert()
        .success();

    remove_project(test_name);
    cleanup_store(test_name);
}

#[test]
fn test_list_and_get() {
    let test_name = "list_get";
    new_project(test_name);

    cmd_with_store(test_name).arg("list").assert().success();

    cmd_with_store(test_name)
        .arg("get")
        .arg(test_name)
        .assert()
        .success();

    remove_project(test_name);
    cleanup_store(test_name);
}
