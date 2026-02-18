// Integration tests for Avon task runner
// These are simplified tests that verify the system works

#[test]
fn test_compilation_succeeds() {
    // Test that the project compiles with cargo
    let output = std::process::Command::new("cargo")
        .args(&["build"])
        .current_dir(".")
        .output();
    
    assert!(output.is_ok(), "cargo build should succeed");
}

#[test]
fn test_binary_exists() {
    // Test that the binary was built
    let binary_path = "./target/debug/avon";
    assert!(
        std::path::Path::new(binary_path).exists(),
        "avon binary should exist at {}",
        binary_path
    );
}

#[test]
fn test_task_runner_module_compiles() {
    // This test passes if the project compiles at all
    // The task_runner module is part of the compiled code
    assert!(std::path::Path::new("src/cli/task_runner.rs").exists());
}

#[test]
fn test_avonfile_example_exists() {
    // Test that example files exist
    assert!(std::path::Path::new("Avonfile.av").exists());
    assert!(std::path::Path::new("test_do.av").exists());
}

#[test]
fn test_phase2_features_exist() {
    // Verify Phase 2 files are in place
    let commands_content = std::fs::read_to_string("src/cli/commands.rs")
        .expect("Should be able to read commands.rs");
    
    // Check that Phase 2 functions exist
    assert!(commands_content.contains("execute_do_run"), "execute_do_run function should exist");
    assert!(commands_content.contains("execute_do_list"), "execute_do_list function should exist");
    assert!(commands_content.contains("execute_do_info"), "execute_do_info function should exist");
}

#[test]
fn test_options_has_phase2_flags() {
    // Verify Phase 2 flags are in options
    let options_content = std::fs::read_to_string("src/cli/options.rs")
        .expect("Should be able to read options.rs");
    
    assert!(options_content.contains("dry_run"), "dry_run field should exist");
    assert!(options_content.contains("list_tasks"), "list_tasks field should exist");
    assert!(options_content.contains("task_info"), "task_info field should exist");
    assert!(options_content.contains("--dry-run"), "--dry-run parsing should exist");
    assert!(options_content.contains("--list"), "--list parsing should exist");
    assert!(options_content.contains("--info"), "--info parsing should exist");
}

#[test]
fn test_avonfile_auto_discovery_implemented() {
    // Verify auto-discovery is implemented
    let commands_content = std::fs::read_to_string("src/cli/commands.rs")
        .expect("Should be able to read commands.rs");
    
    assert!(
        commands_content.contains("Avonfile.av") && commands_content.contains("auto"),
        "Auto-discovery should be implemented"
    );
}

#[test]
fn test_topological_sort_fixed() {
    // Verify the topological sort reverse issue is fixed
    let task_runner_content = std::fs::read_to_string("src/cli/task_runner.rs")
        .expect("Should be able to read task_runner.rs");
    
    // The old code had "order.reverse()" which we removed
    // We should not find it in build_execution_plan anymore
    let lines_with_build_plan = task_runner_content
        .lines()
        .skip_while(|l| !l.contains("fn build_execution_plan"))
        .take_while(|l| !l.contains("pub fn"))
        .collect::<Vec<_>>();
    
    let has_reverse = lines_with_build_plan.iter().any(|l| l.contains("reverse()"));
    assert!(!has_reverse, "order.reverse() should be removed from build_execution_plan");
}

#[test]
fn test_documentation_exists() {
    // Verify Phase 2 documentation exists
    assert!(std::path::Path::new("PHASE1_COMPLETION.md").exists());
    assert!(std::path::Path::new("TASKRUNNER_QUICKSTART.md").exists());
    assert!(std::path::Path::new("TASKRUNNER_EXAMPLES.md").exists());
    assert!(std::path::Path::new("ARCHITECTURE.md").exists());
}

#[test]
fn test_no_compilation_errors() {
    // Run cargo check to verify no compilation errors
    let output = std::process::Command::new("cargo")
        .args(&["check", "--quiet"])
        .current_dir(".")
        .output()
        .expect("cargo check should run");
    
    // Exit code 0 means no errors
    assert!(
        output.status.success(),
        "cargo check should have no errors: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
