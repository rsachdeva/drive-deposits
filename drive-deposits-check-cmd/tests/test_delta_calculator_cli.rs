use anyhow::Result;
use assert_cmd::Command;
use enable_tracing::initialize_test_span;
use predicates::prelude::predicate;
use tracing::debug;

mod enable_tracing;

// with event bridge aws deployment using localstack
#[cfg(feature = "localstack_aws_deploy")]
#[test]
fn test_portfolio_request_two_banks_json_valid() -> Result<()> {
    initialize_test_span("test_portfolio_request_two_banks_json_valid").in_scope(|| {
        let json_request_file_path = "tests/data/portfolio_request_two_banks_json_valid.json";
        let cmd = Command::cargo_bin("drive-deposits-check-cmd")?
            // .env(
            //     "RUST_LOG",
            //     "test=debug,drive_deposits_check_cmd=debug,drive_deposits_cal_types=debug",
            // )
            // SEND_CAL_EVENTS = "true"
            // only when manually want to test
            // actually sends events to localstack aws event bridge
            // test will fail if SEND_CAL_EVENTS is true and event bridge is not set up
            .env("SEND_CAL_EVENTS", "true")
            .env("USE_LOCALSTACK", "true")
            .arg(json_request_file_path)
            .assert()
            .success();

        let output = cmd.get_output();
        debug!(
            "Command Stdout is: {}",
            String::from_utf8_lossy(&output.stdout)
        );
        //
        // debug!(
        //     "Command Stderr is: {}",
        //     String::from_utf8_lossy(&output.stderr)
        // );
        // assert!(output.stderr.is_empty());

        cmd.stdout(predicate::str::contains("VISION-BANK"))
            .stdout(predicate::str::contains(
                "tests/data/portfolio_request_two_banks_json_valid.json",
            ))
            .stdout(predicate::str::contains(
                "\"maturity_date_in_bank_tz\":\"2044-02-11\"",
            ));

        Ok(())
    })
}

#[cfg(not(feature = "localstack_aws_deploy"))]
#[test]
fn test_portfolio_request_two_banks_json_invalid() -> Result<()> {
    initialize_test_span("portfolio_request_two_banks_json_invalid").in_scope(|| {
        let json_request_file_path = "tests/data/portfolio_request_two_banks_json_invalid.json";
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
