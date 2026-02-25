use benchfx::task::run_once;

#[test]
fn valid_command_succeeds() {
    let result = run_once("echo hi");
    assert!(result.success);
    assert!(result.duration_ms >= 0.0);
}

#[test]
fn invalid_command_fails() {
    let result = run_once("this_command_should_not_exist_12345");
    assert!(!result.success);
    assert!(result.duration_ms >= 0.0);
}

#[test]
fn empty_command_fails() {
    let result = run_once("");
    assert!(!result.success);
    assert!(result.duration_ms >= 0.0);
}

#[test]
fn non_zero_exit_code_is_failure() {
    let result = run_once("cmd /C exit 1");
    assert!(!result.success);
    assert!(result.duration_ms >= 0.0);
}