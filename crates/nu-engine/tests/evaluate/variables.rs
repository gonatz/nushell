use nu_test_support::fs::Stub::FileWithContent;
use nu_test_support::fs::{AbsolutePath, DisplayPath};
use nu_test_support::playground::{says, Playground};

use hamcrest2::assert_that;
use hamcrest2::prelude::*;

#[test]
fn config_path_variable_present() {
    Playground::setup("nu_variable_test_1", |_, nu| {
        assert_that!(
            nu.pipeline("echo $nu.config-path"),
            says().to_stdout(nu.get_config())
        );
    })
}

#[test]
fn custom_config_path_variable_present() {
    Playground::setup("nu_variable_test_2", |dirs, nu| {
        let file = AbsolutePath::new(dirs.test().join("config.toml"));

        nu.with_config(&file);
        nu.with_files(vec![FileWithContent(
            "config.toml",
            "skip_welcome_message = true",
        )]);

        assert_that!(
            nu.pipeline("echo $nu.config-path"),
            says().to_stdout(&file.display_path())
        );
    })
}
