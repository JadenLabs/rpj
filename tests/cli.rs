use assert_cmd::Command;
use std::fs;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("rpj").unwrap();
    cmd.arg("--help").assert().failure();
}

fn remove_project(name: &str) {
    let mut cmd = Command::cargo_bin("rpj").unwrap();
    cmd.arg("remove").arg(name).assert().success();
}

fn new_project(name: &str, directory: &str) {
    let mut new_cmd = Command::cargo_bin("rpj").unwrap();
    new_cmd
        .arg("new")
        .args([
            name,
            directory,
            "--run-cmd",
            "echo 'Hello, World!'",
            "--description",
            "Test project",
        ])
        .assert()
        .success();
}

#[test]
fn test_new_update_remove() {
    new_project("test_new_rm", "./tests");

    let mut update_cmd = Command::cargo_bin("rpj").unwrap();
    update_cmd
        .arg("update")
        .args(["test_new_rm", "--description", "Updated description"])
        .assert()
        .success();

    remove_project("test_new_rm");
}

#[test]
fn test_export_and_add() {
    fs::create_dir_all("./tests/export_add").unwrap();
    new_project("test_export_add", "./tests/export_add");

    let mut export_cmd = Command::cargo_bin("rpj").unwrap();
    export_cmd
        .arg("export")
        .args(["test_export_add", "--export-path", "./tests/export_add"])
        .assert()
        .success();

    remove_project("test_export_add");

    let mut add_cmd = Command::cargo_bin("rpj").unwrap();
    add_cmd
        .arg("add")
        .args(["./tests/export_add/test_export_add.rpj", "--delete-after"])
        .assert()
        .success();

    remove_project("test_export_add");
    fs::remove_dir("./tests/export_add").unwrap();
}
