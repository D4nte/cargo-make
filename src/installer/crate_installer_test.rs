use super::*;

#[test]
fn invoke_rustup_install_none() {
    let info = InstallCrateInfo {
        crate_name: "bad_crate_name".to_string(),
        binary: "test".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: None,
    };

    let output = invoke_rustup_install(&info);
    assert!(!output);
}

#[test]
fn invoke_rustup_install_fail() {
    let info = InstallCrateInfo {
        crate_name: "bad_crate_name".to_string(),
        binary: "test".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: Some("unknown_rustup_component_test".to_string()),
    };

    let output = invoke_rustup_install(&info);
    assert!(!output);
}

#[test]
fn invoke_cargo_install_test() {
    let info = InstallCrateInfo {
        crate_name: "bad_crate_name".to_string(),
        binary: "cargo_bad".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: Some("unknown_rustup_component_test".to_string()),
    };

    invoke_cargo_install(&info, &None, false);
}

#[test]
fn install_test() {
    let info = InstallCrateInfo {
        crate_name: "bad_crate_name".to_string(),
        binary: "cargo_bad".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: Some("unknown_rustup_component_test".to_string()),
    };

    install(&info, &None, false);
}