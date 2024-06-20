use std::path::PathBuf;
use swc_core::ecma::{
    parser::{EsConfig, Syntax},
    transforms::testing::{test_fixture, FixtureTestConfig},
    visit::as_folder,
};
use testing::fixture;

use next_linaria_plugin::TransformVisitor;

#[fixture("tests/fixture/page/code.js")]
fn fixture_page(input: PathBuf) {
    let output = input.with_file_name("output.js");
    let filename = input.to_string_lossy().to_string();

    test_fixture(
        Syntax::Es(EsConfig {
            jsx: true,
            ..Default::default()
        }),
        &|_| {
            as_folder(TransformVisitor {
                file_name: filename.clone(),
                import_name: None,
            })
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}
