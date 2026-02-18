// Integration tests for Avon task runner (do mode)
// Tests the full CLI flow by running the avon binary with real .av files

use std::fs;
use std::path::Path;
use std::process::Command;

const AVON_BIN: &str = "./target/debug/avon";
const FIXTURES_DIR: &str = "test_fixtures_integration";

/// Setup: create the test fixtures directory
fn setup() {
    fs::create_dir_all(FIXTURES_DIR).ok();
}

/// Create a test .av file and return its path
fn write_fixture(name: &str, content: &str) -> String {
    setup();
    let path = format!("{}/{}", FIXTURES_DIR, name);
    fs::write(&path, content).expect("write fixture");
    path
}

/// Run avon with the given arguments and return (success, stdout, stderr)
fn avon(args: Vec<&str>) -> (bool, String, String) {
    let output = Command::new(AVON_BIN)
        .args(args)
        .output()
        .expect("failed to run avon binary");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

// ==================== Binary & build checks ====================

#[test]
fn test_binary_exists() {
    assert!(
        Path::new(AVON_BIN).exists(),
        "avon binary should exist at {}",
        AVON_BIN
    );
}

#[test]
fn test_no_compilation_errors() {
    let output = Command::new("cargo")
        .args(["check", "--quiet"])
        .current_dir(".")
        .output()
        .expect("cargo check should run");
    assert!(
        output.status.success(),
        "cargo check should pass: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

// ==================== Simple task execution ====================

#[test]
fn test_simple_task_execution() {
    let file = write_fixture("simple.av", r#"{ hello: "echo 'Hello from avon!'" }"#);
    let (ok, stdout, _) = avon(vec!["do", "hello", &file]);
    assert!(ok, "simple task should succeed");
    assert!(stdout.contains("Running: hello"));
    assert!(stdout.contains("Hello from avon!"));
    assert!(stdout.contains("completed successfully"));
}

// ==================== Dependencies ====================

#[test]
fn test_linear_dependency_execution() {
    let file = write_fixture(
        "linear_deps.av",
        r#"{
  prepare: "echo 'STEP1'",
  build: {cmd: "echo 'STEP2'", deps: ["prepare"]},
  deploy: {cmd: "echo 'STEP3'", deps: ["build"]}
}"#,
    );
    let (ok, stdout, _) = avon(vec!["do", "deploy", &file]);
    assert!(ok);
    // Verify execution order
    let p1 = stdout.find("STEP1").expect("STEP1");
    let p2 = stdout.find("STEP2").expect("STEP2");
    let p3 = stdout.find("STEP3").expect("STEP3");
    assert!(p1 < p2, "prepare before build");
    assert!(p2 < p3, "build before deploy");
}

#[test]
fn test_diamond_dependency_execution() {
    let file = write_fixture(
        "diamond.av",
        r#"{
  a: "echo 'A'",
  b: {cmd: "echo 'B'", deps: ["a"]},
  c: {cmd: "echo 'C'", deps: ["a"]},
  d: {cmd: "echo 'D'", deps: ["b", "c"]}
}"#,
    );
    let (ok, stdout, _) = avon(vec!["do", "d", &file]);
    assert!(ok);
    // 'a' should appear before both 'b' and 'c'
    let a_pos = stdout.find("Running: a").expect("should have a");
    let b_pos = stdout.find("Running: b").expect("should have b");
    let c_pos = stdout.find("Running: c").expect("should have c");
    assert!(a_pos < b_pos && a_pos < c_pos);
    // 'a' should only run once
    assert_eq!(stdout.matches("Running: a").count(), 1);
}

// ==================== --dry-run ====================

#[test]
fn test_dry_run_shows_plan_without_executing() {
    let file = write_fixture(
        "dryrun.av",
        r#"{
  step1: "echo 'SHOULD_NOT_APPEAR'",
  step2: {cmd: "echo 'ALSO_NOT'", deps: ["step1"]}
}"#,
    );
    let (ok, stdout, _) = avon(vec!["do", "--dry-run", "step2", &file]);
    assert!(ok);
    assert!(stdout.contains("Execution Plan"));
    assert!(stdout.contains("step1"));
    assert!(stdout.contains("step2"));
    // Should NOT actually execute the tasks - no "Running:" prefix
    assert!(
        !stdout.contains("Running:"),
        "dry-run should not actually run tasks"
    );
}

// ==================== --list ====================

#[test]
fn test_list_shows_all_tasks() {
    let file = write_fixture(
        "list.av",
        r#"{
  build: {cmd: "cargo build", desc: "Build the project"},
  test: {cmd: "cargo test", desc: "Run tests"},
  clean: "cargo clean"
}"#,
    );
    let (ok, stdout, _) = avon(vec!["do", "--list", &file]);
    assert!(ok);
    assert!(stdout.contains("Available Tasks"));
    assert!(stdout.contains("build"));
    assert!(stdout.contains("Build the project"));
    assert!(stdout.contains("test"));
    assert!(stdout.contains("Run tests"));
    assert!(stdout.contains("clean"));
}

#[test]
fn test_list_empty_taskfile() {
    let file = write_fixture("empty.av", "{}");
    let (ok, stdout, _) = avon(vec!["do", "--list", &file]);
    assert!(ok);
    assert!(stdout.contains("No tasks found"));
}

// ==================== --info ====================

#[test]
fn test_info_shows_task_details() {
    let file = write_fixture(
        "info.av",
        r#"{
  compile: {cmd: "gcc main.c", desc: "Compile C code"},
  link: {cmd: "ld a.o", deps: ["compile"]}
}"#,
    );
    let (ok, stdout, _) = avon(vec!["do", "--info", "link", &file]);
    assert!(ok);
    assert!(stdout.contains("Task: link"));
    assert!(stdout.contains("ld a.o"));
    assert!(stdout.contains("compile"));
}

#[test]
fn test_info_shows_env_vars() {
    let file = write_fixture(
        "info_env.av",
        r#"{
  deploy: {cmd: "deploy.sh", env: {ENV: "prod", VERSION: "1.0"}}
}"#,
    );
    let (ok, stdout, _) = avon(vec!["do", "--info", "deploy", &file]);
    assert!(ok);
    assert!(stdout.contains("Environment Variables"));
}

// ==================== Error handling ====================

#[test]
fn test_task_not_found_error() {
    let file = write_fixture("notfound.av", r#"{ real: "echo hi" }"#);
    let (ok, _, stderr) = avon(vec!["do", "fake_task", &file]);
    assert!(!ok, "should fail for nonexistent task");
    assert!(stderr.contains("not found"));
}

#[test]
fn test_typo_suggestion_in_error() {
    let file = write_fixture(
        "typo.av",
        r#"{ build: "echo build", test: "echo test", deploy: "echo deploy" }"#,
    );
    let (ok, _, stderr) = avon(vec!["do", "bild", &file]);
    assert!(!ok);
    assert!(
        stderr.contains("Did you mean 'build'"),
        "should suggest 'build' for typo 'bild', got: {}",
        stderr
    );
}

#[test]
fn test_cyclic_dependency_error() {
    let file = write_fixture(
        "cycle.av",
        r#"{
  a: {cmd: "echo a", deps: ["b"]},
  b: {cmd: "echo b", deps: ["a"]}
}"#,
    );
    let (ok, _, stderr) = avon(vec!["do", "a", &file]);
    assert!(!ok);
    assert!(
        stderr.contains("Cyclic") || stderr.contains("cycle"),
        "should report cyclic dependency, got: {}",
        stderr
    );
}

#[test]
fn test_undefined_dependency_error() {
    let file = write_fixture(
        "undef_dep.av",
        r#"{ task: {cmd: "echo hi", deps: ["missing"]} }"#,
    );
    let (ok, _, stderr) = avon(vec!["do", "task", &file]);
    assert!(!ok);
    assert!(
        stderr.contains("missing") || stderr.contains("does not exist"),
        "should report undefined dependency, got: {}",
        stderr
    );
}

#[test]
fn test_undefined_dep_typo_suggestion() {
    let file = write_fixture(
        "dep_typo.av",
        r#"{
  build: "echo building",
  test: {cmd: "echo testing", deps: ["bild"]}
}"#,
    );
    let (ok, _, stderr) = avon(vec!["do", "test", &file]);
    assert!(!ok);
    assert!(
        stderr.contains("Did you mean 'build'"),
        "should suggest 'build' for dependency typo 'bild', got: {}",
        stderr
    );
}

// ==================== Environment variables ====================

#[test]
fn test_task_env_vars_passed_to_command() {
    let file = write_fixture(
        "env_vars.av",
        r#"{
  greet: {
    cmd: "echo Hello $GREETING_NAME",
    env: {GREETING_NAME: "World"}
  }
}"#,
    );
    let (ok, stdout, _) = avon(vec!["do", "greet", &file]);
    assert!(ok);
    assert!(
        stdout.contains("Hello World"),
        "env var should be expanded, got: {}",
        stdout
    );
}

#[test]
fn test_task_env_vars_braces_syntax() {
    let file = write_fixture(
        "env_braces.av",
        r#"{
  show: {
    cmd: "echo ${APP}_v${VER}",
    env: {APP: "myapp", VER: "2"}
  }
}"#,
    );
    let (ok, stdout, _) = avon(vec!["do", "show", &file]);
    assert!(ok);
    assert!(
        stdout.contains("myapp_v2"),
        "braces env var syntax should work, got: {}",
        stdout
    );
}

// ==================== Complex pipelines ====================

#[test]
fn test_complex_pipeline_dry_run() {
    let file = write_fixture(
        "pipeline.av",
        r#"{
  checkout: "echo checkout",
  build: {cmd: "echo build", deps: ["checkout"]},
  unit_test: {cmd: "echo unit", deps: ["build"]},
  integration_test: {cmd: "echo integration", deps: ["build"]},
  package: {cmd: "echo package", deps: ["unit_test", "integration_test"]},
  deploy: {cmd: "echo deploy", deps: ["package"]}
}"#,
    );
    let (ok, stdout, _) = avon(vec!["do", "--dry-run", "deploy", &file]);
    assert!(ok);

    // Verify order: checkout must appear before build, etc.
    let checkout_pos = stdout.find("checkout").expect("checkout");
    let build_pos = stdout.find("build").expect("build");
    let deploy_pos = stdout.find("deploy").expect("deploy");
    assert!(checkout_pos < build_pos);
    assert!(build_pos < deploy_pos);
}

#[test]
fn test_task_with_description_displayed() {
    let file = write_fixture(
        "desc_run.av",
        r#"{ greet: {cmd: "echo hi", desc: "Say hello"} }"#,
    );
    let (ok, stdout, _) = avon(vec!["do", "greet", &file]);
    assert!(ok);
    assert!(
        stdout.contains("Say hello"),
        "description should be printed during run"
    );
}

// ==================== Failing tasks ====================

#[test]
fn test_failing_task_returns_error() {
    let file = write_fixture("fail.av", r#"{ broken: "exit 1" }"#);
    let (ok, _, stderr) = avon(vec!["do", "broken", &file]);
    assert!(!ok, "task with 'exit 1' should fail");
    assert!(
        stderr.contains("failed") || stderr.contains("Error"),
        "stderr should mention failure, got: {}",
        stderr
    );
}

#[test]
fn test_failing_dep_stops_execution() {
    let file = write_fixture(
        "fail_dep.av",
        r#"{
  step1: "exit 1",
  step2: {cmd: "echo should_not_run", deps: ["step1"]}
}"#,
    );
    let (ok, stdout, _) = avon(vec!["do", "step2", &file]);
    assert!(!ok);
    assert!(
        !stdout.contains("should_not_run"),
        "step2 should not run when step1 fails"
    );
}

// ==================== Phase 3 code structure verification ====================

#[test]
fn test_phase3_features_in_code() {
    let task_runner = fs::read_to_string("src/cli/task_runner.rs").unwrap();
    // Phase 3: expand_env_vars function exists
    assert!(task_runner.contains("expand_env_vars"));
    // Phase 3: typo suggestions in TaskNotFound
    assert!(task_runner.contains("suggestions"));
    // Phase 3: ParseError variant is used
    let commands = fs::read_to_string("src/cli/commands.rs").unwrap();
    assert!(commands.contains("TaskError::ParseError"));
}

#[test]
fn test_avon_file_exists() {
    assert!(Path::new("Avon.av").exists());
}

#[test]
fn test_topological_sort_no_reverse() {
    // Verify the old buggy order.reverse() is not present
    let src = fs::read_to_string("src/cli/task_runner.rs").unwrap();
    let in_build_plan: String = src
        .lines()
        .skip_while(|l| !l.contains("fn build_execution_plan"))
        .take_while(|l| !l.starts_with("    /// ") || l.contains("Build execution"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !in_build_plan.contains("reverse()"),
        "order.reverse() should not be in build_execution_plan"
    );
}
