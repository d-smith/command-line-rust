use assert_cmd::Command;


#[test] // test attribute
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}