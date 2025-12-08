use benchx::cli::Config;
use clap::Parser;

#[test]
fn parses_required_flags() {
    let args = [
        "benchx",
        "--command", "echo hi",
        "--iterations", "5",
    ];

    let config = Config::try_parse_from(&args).expect("expected args to parse");

    assert_eq!(config.command, "echo hi");
    assert_eq!(config.iterations, 5);
    assert_eq!(config.concurrency, 1);
    assert_eq!(config.timeout, 1000);
}

#[test]
fn parses_all_flags_with_custom_values() {
    let args = [
        "benchx",
        "--command", "curl https://api.github.com",
        "--iterations", "10",
        "--concurrency", "4",
        "--timeout", "5000",
    ];

    let config = Config::try_parse_from(&args).expect("expected args to parse");

    assert_eq!(config.command, "curl https://api.github.com");
    assert_eq!(config.iterations, 10);
    assert_eq!(config.concurrency, 4);
    assert_eq!(config.timeout, 5000);
}

#[test]
fn missing_iterations_errors() {
    let args = [
        "benchx",
        "--command", "echo hi",
    ];

    let result = Config::try_parse_from(&args);
    assert!(result.is_err());
}

#[test]
fn missing_command_errors() {
    let args = [
        "benchx",
        "--iterations", "5",
    ];

    let result = Config::try_parse_from(&args);
    assert!(result.is_err());
}