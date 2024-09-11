use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::predicate;
use tracing::debug;

use enable_tracing::initialize_test_span;

mod enable_tracing;

// this test actually can be uncommented to check aws deploy works
// #[test]
// fn test_valid_two_banks_json_request() -> Result<()> {
//     initialize_test_span("test_two_banks_json_request").in_scope(|| {
//         let json_request_file_path = "tests/data/two_banks_json_request_valid.json";
//         let cmd = Command::cargo_bin("drive-deposits-check-cmd")?
//             // .env("RUST_LOG", "test=debug,drive_deposits_check_cmd=debug")
//             .arg(json_request_file_path)
//             .assert()
//             .success();
//
//         // let output = cmd.get_output();
//         // debug!(
//         //     "Command Stdout is: {}",
//         //     String::from_utf8_lossy(&output.stdout)
//         // );
//         //
//         // debug!(
//         //     "Command Stderr is: {}",
//         //     String::from_utf8_lossy(&output.stderr)
//         // );
//         // assert!(output.stderr.is_empty());
//
//         cmd.stdout(predicate::str::contains("VISION-BANK"))
//             .stdout(predicate::str::contains(
//                 "tests/data/two_banks_json_request_valid.json",
//             ))
//             .stdout(predicate::str::contains(
//                 "\"maturity_date_in_bank_tz\":\"2044-02-11\"",
//             ));
//
//         Ok(())
//     })
// }

#[test]
fn test_invalid_two_banks_json_request() -> Result<()> {
    initialize_test_span("test_invalid_two_banks_json_request").in_scope(|| {
        let json_request_file_path = "tests/data/two_banks_json_request_invalid.json";
        let cmd = Command::cargo_bin("drive-deposits-check-cmd")?
            .arg(json_request_file_path)
            .assert()
            .failure()
            .stderr(predicate::str::contains("Must be a valid timezone"))
            .stderr(predicate::str::contains(
                "Must be Day, Week, Month, or Year",
            ));

        let output = cmd.get_output();
        debug!(
            "Command Stdout is: {}",
            String::from_utf8_lossy(&output.stdout)
        );

        debug!(
            "Command Stderr is: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        Ok(())
    })
}
