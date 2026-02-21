//! CI workflow configuration tests
//!
//! These tests verify that the CI workflow YAML contains
//! the necessary configuration for MSRV compatibility.

use std::fs;

/// Path to the CI workflow file
const CI_WORKFLOW_PATH: &str = ".github/workflows/ci.yml";

// ==========================================================
// File existence tests
// ==========================================================

#[test]
fn test_ci_workflow_file_exists() {
    // Given: The CI workflow path
    // When: Checking file existence
    let exists = fs::metadata(CI_WORKFLOW_PATH).is_ok();

    // Then: The file should exist
    assert!(
        exists,
        "CI workflow file should exist at {CI_WORKFLOW_PATH}"
    );
}

// ==========================================================
// MSRV matrix configuration tests
// ==========================================================

#[test]
fn test_ci_workflow_contains_msrv_matrix() {
    // Given: The CI workflow content
    let content =
        fs::read_to_string(CI_WORKFLOW_PATH).expect("Should be able to read CI workflow file");

    // When: Checking for MSRV in matrix
    // Then: It should contain 1.70.0
    assert!(
        content.contains("1.70.0"),
        "CI workflow should test against MSRV 1.70.0"
    );
}

#[test]
fn test_ci_workflow_contains_stable_in_matrix() {
    // Given: The CI workflow content
    let content =
        fs::read_to_string(CI_WORKFLOW_PATH).expect("Should be able to read CI workflow file");

    // When: Checking for stable in matrix
    // Then: It should contain stable
    assert!(
        content.contains("stable"),
        "CI workflow should test against stable Rust"
    );
}

// ==========================================================
// MSRV compatibility step tests
// ==========================================================

#[test]
fn test_ci_workflow_contains_lockfile_regeneration() {
    // Given: The CI workflow content
    let content =
        fs::read_to_string(CI_WORKFLOW_PATH).expect("Should be able to read CI workflow file");

    // When: Checking for lockfile regeneration step
    // Then: It should contain cargo generate-lockfile
    assert!(
        content.contains("cargo generate-lockfile"),
        "CI workflow should have a step to regenerate Cargo.lock for MSRV"
    );
}

#[test]
fn test_ci_workflow_lockfile_step_is_conditional() {
    // Given: The CI workflow content
    let content =
        fs::read_to_string(CI_WORKFLOW_PATH).expect("Should be able to read CI workflow file");

    // When: Checking for the condition on the lockfile step
    // Then: It should only run for non-stable (MSRV) toolchains
    assert!(
        content.contains("matrix.rust != 'stable'"),
        "Lockfile regeneration should only run for MSRV, not stable"
    );
}

// ==========================================================
// MSRV consistency tests
// ==========================================================

#[test]
fn test_cargo_toml_msrv_matches_ci_matrix() {
    // Given: Both Cargo.toml and CI workflow
    let cargo_toml = fs::read_to_string("Cargo.toml").expect("Should be able to read Cargo.toml");
    let ci_workflow =
        fs::read_to_string(CI_WORKFLOW_PATH).expect("Should be able to read CI workflow file");

    // When: Extracting MSRV from Cargo.toml
    // Then: The same version should appear in CI matrix
    assert!(
        cargo_toml.contains("rust-version = \"1.70.0\""),
        "Cargo.toml should define MSRV as 1.70.0"
    );
    assert!(
        ci_workflow.contains("\"1.70.0\""),
        "CI matrix should include MSRV 1.70.0"
    );
}

// ==========================================================
// CI workflow structure tests (failure cases)
// ==========================================================

#[test]
fn test_ci_workflow_is_not_empty() {
    // Given: The CI workflow content
    let content =
        fs::read_to_string(CI_WORKFLOW_PATH).expect("Should be able to read CI workflow file");

    // When: Checking content length
    // Then: It should not be empty
    assert!(!content.is_empty(), "CI workflow file should not be empty");
}

#[test]
fn test_ci_workflow_contains_test_step() {
    // Given: The CI workflow content
    let content =
        fs::read_to_string(CI_WORKFLOW_PATH).expect("Should be able to read CI workflow file");

    // When: Checking for test execution step
    // Then: It should contain cargo test
    assert!(
        content.contains("cargo test"),
        "CI workflow should have a cargo test step"
    );
}

#[test]
fn test_ci_workflow_lockfile_step_before_test_step() {
    // Given: The CI workflow content
    let content =
        fs::read_to_string(CI_WORKFLOW_PATH).expect("Should be able to read CI workflow file");

    // When: Checking order of steps
    let lockfile_pos = content
        .find("cargo generate-lockfile")
        .expect("cargo generate-lockfile should exist");
    let test_pos = content.find("cargo test").expect("cargo test should exist");

    // Then: Lockfile regeneration should come before test execution
    assert!(
        lockfile_pos < test_pos,
        "cargo generate-lockfile step should appear before cargo test step"
    );
}
