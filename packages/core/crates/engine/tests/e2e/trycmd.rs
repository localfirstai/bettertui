#[test]
fn e2e_render_demo() {
    trycmd::TestCases::new()
        .default_bin_path(trycmd::cargo::cargo_bin("render_demo"))
        .case("tests/e2e/cmd/*.trycmd");
}
