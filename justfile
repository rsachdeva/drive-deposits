# clean
clean:
    cargo clean

clean-with-lambdas: clean-build-drive-deposits-dynamodb-queries clean-build-drive-deposit-event-rules clean-build-drive-deposit-event-bus
    cargo clean

#  only for checking; could use cargo build --workspace --tests instead also

# for further confirmation -- keeps binary separate from tests still
check-with-tests:
    cargo check --workspace --tests

# build must have default-members or pass --workspace
build-with-tests:
    cargo build --workspace --tests

build:
    cargo build --workspace

build-with-lambdas: build-drive-deposits-event-bus build-drive-deposits-event-rules build-drive-deposits-dynamodb-queries
    cargo build --workspace --tests

udeps:
    cargo +nightly udeps --workspace

build-drive-deposits-cal-types:
    cd drive-deposits-cal-types && \
    cargo build --package drive-deposits-cal-types

build-drive-deposits-proto-grpc-types:
    cd drive-deposits-proto-grpc-types && \
    cargo build --package drive-deposits-proto-grpc-types

build-drive-deposits-rest-types:
    cd drive-deposits-rest-types && \
    cargo build --package drive-deposits-rest-types

# See README.md for more details.
build-drive-deposits-check-cmd:
    cd drive-deposits-check-cmd && \
    cargo build --package drive-deposits-check-cmd

build-drive-deposits-grpc-server:
    cd drive-deposits-grpc-server && \
    cargo build --package drive-deposits-grpc-server

build-drive-deposits-rest-gateway-server:
    cd drive-deposits-rest-gateway-server && \
    cargo build --package drive-deposits-rest-gateway-server --tests

# watching builds

# .cargo/cargo.toml has SEND_CAL_EVENTS = "true" so we can overrride it here as needed
watch-build:
    cargo watch -x "build --workspace"

watch-build-with-tests:
    cargo watch -x "build --workspace --tests"

watch-build-drive-deposits-cal-types:
    cd drive-deposits-cal-types
    cargo watch -x "build --package drive-deposits-cal-types"

watch-build-drive-deposits-event-source:
    cd drive-deposits-event-source
    cargo watch -x "build --package drive-deposits-event-source"

watch-build-drive-deposits-rest-gateway-server-with-tests:
    cd drive-deposits-rest-gateway-server && \
    cargo watch -x "build --package drive-deposits-rest-gateway-server --tests"

watch-build-drive-deposits-grpc-server:
    cd drive-deposits-grpc-server
    cargo watch -x "build --package drive-deposits-grpc-server"

# .cargo/cargo.toml has SEND_CAL_EVENTS = "true" so we can overrride it here as needed

# run only
run-drive-deposits-check-cmd-valid:
    SEND_CAL_EVENTS="false" cargo ddcheck -- drive-deposits-rest-gateway-server/data/portfolio_request_valid.json

run-drive-deposits-check-cmd-help:
    SEND_CAL_EVENTS="true" cargo ddcheck -- --help

run-drive-deposits-check-cmd-valid-send-events:
    SEND_CAL_EVENTS="true" cargo ddcheck -- drive-deposits-rest-gateway-server/data/portfolio_request_valid.json

run-drive-deposits-check-cmd-valid-send-events-lesser-amount-investments:
    SEND_CAL_EVENTS="true" cargo ddcheck -- drive-deposits-rest-gateway-server/data/portfolio_request_valid_lesser_amount_investments.json

run-drive-deposits-check-cmd-valid-send-events-greater-amount-investments:
    SEND_CAL_EVENTS="true" cargo ddcheck -- drive-deposits-rest-gateway-server/data/portfolio_request_valid_greater_amount_investments.json

run-drive-deposits-check-cmd-invalid-decimal:
    SEND_CAL_EVENTS="false" cargo ddcheck -- drive-deposits-rest-gateway-server/data/portfolio_request_invalid_decimal.json

run-drive-deposits-check-cmd-invalid:
    SEND_CAL_EVENTS="false" cargo ddcheck -- drive-deposits-rest-gateway-server/data/portfolio_request_invalid_period_unit_account_type_decimal_bank_tz_start_date.json

run-drive-deposits-grpc-server:
    SEND_CAL_EVENTS="true" cargo run --package drive-deposits-grpc-server --bin drive-deposits-grpc-server

run-drive-deposits-rest-grpc-gateway-server:
    cargo run --package drive-deposits-rest-gateway-server --bin drive-deposits-rest-gateway-server

# .cargo/cargo.toml has SEND_CAL_EVENTS = "true" so we can overrride it here as needed

# watch running
watch-run-drive-deposits-check-cmd-valid:
    SEND_CAL_EVENTS="false" cargo watch -x "ddcheck -- examples/data/rest_server_as_gateway_request_valid.json"

watch-run-drive-deposits-check-cmd-valid-send-events:
    SEND_CAL_EVENTS="true" cargo watch -x "ddcheck -- examples/data/rest_server_as_gateway_request_valid.json"

watch-run-drive-deposits-check-cmd-invalid-decimal:
    SEND_CAL_EVENTS="false" cargo watch -x "ddcheck -- examples/data/rest_server_as_gateway_request_invalid_decimal.json"

watch-run-drive-deposits-check-cmd-invalid:
    SEND_CAL_EVENTS="false" cargo watch -x "ddcheck -- examples/data/rest_server_as_gateway_request_invalid_period_unit_account_type_decimal_bank_tz_start_date.json"

watch-run-drive-deposits-grpc-server:
    cargo watch --why --poll  -x "run --package drive-deposits-grpc-server --bin drive-deposits-grpc-server"

watch-run-drive-deposits-rest-gateway-server:
    cargo watch --why --poll  -x "run --package drive-deposits-rest-gateway-server --bin drive-deposits-rest-gateway-server"

# test
# cargo help test
# cargo test -- --help
# With --nocapture, since tests run in parallel by default, the output from different tests would indeed be mixed together. This can lead to interleaved output in the console. The --show-output allows to see the stdout from passing tests while maintaining readable, non-interleaved output
# --nocapture:
#
#Shows output in real-time as tests run
#Since tests run in parallel by default, output gets interleaved
#Can be harder to read due to mixed output from concurrent tests
#--show-output:
#
#Shows captured stdout of successful tests
#Maintains clean, readable output organization
#Works well with parallel test execution
# Still shows output for failed tests

test:
    cargo test --workspace --tests -- --show-output

test-intg:
    cargo test --workspace --test '*' -- --show-output

test-rest-gateway-server:
    cargo test --package drive-deposits-rest-gateway-server --tests -- --show-output

# localstart should be started before running this; use just localstack-start
test-e2e: localstack-deploy-drive-deposits-event-rules
    cargo test -p drive-deposits-check-cmd --test test_delta_calculator_cli --features localstack_aws_deploy -- --show-output

# watch test
watch-test:
    cargo watch -x "test --workspace"

# deploy additions
# actual aws deploy (localstack counterpart below)

# event source with bus
validate-drive-deposits-event-bus:
    echo "validating before building DriveDepositsEventBus" && \
    cd drive-deposits-event-source && \
    sam validate --lint --template-file template.yaml

build-drive-deposits-event-bus: validate-drive-deposits-event-bus
    echo "building before deploy DriveDepositsEventBus" && \
    cd drive-deposits-event-source && \
    sam build --template-file template.yaml

deploy-drive-deposits-event-bus: build-drive-deposits-event-bus
    echo "deploy DriveDepositsEventBus" && \
    cd drive-deposits-event-source && \
    sam deploy --no-confirm-changeset --config-env dev || true

# if needed guided; usually needed to create samconfig.yml initially that can be modifed later
deploy-guided-drive-deposits-event-bus: build-drive-deposits-event-bus
    echo "deploy guided DriveDepositsEventBus"&& \
    cd drive-deposits-event-source && \
    sam deploy --guided

# event target with rule
validate-drive-deposits-event-rules:
    echo "validating before building event rules" && \
    cd drive-deposits-logs-lambda-target && \
    sam validate --lint --beta-features --template-file template.yaml

build-drive-deposits-event-rules: validate-drive-deposits-event-rules
    echo "building before deploy event rules" && \
    cd drive-deposits-logs-lambda-target && \
    sam build --beta-features --template-file template.yaml

deploy-drive-deposits-event-rules: deploy-drive-deposits-event-bus build-drive-deposits-event-rules
    echo "deploy event rules"
    cd drive-deposits-logs-lambda-target && \
    sam deploy --no-confirm-changeset --config-env dev --parameter-overrides UseLocalstack="false" Environment="dev" || true

# if needed guided; usually needed to create samconfig.yml initially that can be modifed later
deploy-guided-drive-deposits-event-rules: build-drive-deposits-event-rules
    echo "deploy guided event rules" && \
    cd drive-deposits-logs-lambda-target && \
    sam deploy --guided

aws-invoke-drive-deposits-event-rules-lambda:
    echo "invoking lambda function" && \
    cd drive-deposits-logs-lambda-target && \
    aws lambda invoke --invocation-type=Event --function-name drive_deposits_by_level_lambda_writer_dev --payload file://data/drive-deposits-event-banks-level.json --cli-binary-format raw-in-base64-out out.json

logs-drive-deposits-event-rules-lambda:
    echo "localstack logs" && \
    cd drive-deposits-logs-lambda-target && \
    sam logs --stack-name drive-deposits-event-rules-dev --name DriveDepositsByLevelLambdaFunction -t

# cargo lambda for pure local development start withiout even localstack conatiner
# if needed for cargo lambda without going through sam for just lambda function

# cargo lambda build --release
cargo-lambda-build-drive-deposits-event-rules-lambda:
    echo "building lambda function" && \
    cd drive-deposits-logs-lambda-target && \
    cargo lambda build

cargo-lambda-build-watching-drive-deposits-event-rules-lambda:
    echo "build watching lambda function" && \
    cd drive-deposits-logs-lambda-target && \
    cargo watch -x "lambda build"

# for reader and writer to work together in localstack

localstack_drive_deposit_table_name := "drive-deposits-event-rules-dev-DriveDepositsTable-8bfc05e7"

# RUST_LOG=by_level_lambda_writer=debug not needed as added in .cargo/config.toml

# cargo lambda build --release
cargo-lambda-watch-drive-deposits-event-rules-lambda:
    echo "watching lambda function" && \
    cd drive-deposits-logs-lambda-target && \
    export DRIVE_DEPOSITS_TABLE_NAME={{ localstack_drive_deposit_table_name }} && \
    export USE_LOCALSTACK="true" && \
    echo $DRIVE_DEPOSITS_TABLE_NAME && \
    echo $USE_LOCALSTACK && \
    cargo lambda watch

cargo-lambda-invoke-drive-deposits-event-rules-lambda:
    echo "invoking lambda function" && \
    cd drive-deposits-logs-lambda-target && \
    cargo lambda invoke by_level_lambda_writer --data-file data/drive-deposits-event-banks-level.json

# deployed delete

# event target with rule
deployed-delete-drive-deposits-event-rules:
    echo "deployed delete drive-deposits-event-rules-dev" && \
    cd drive-deposits-logs-lambda-target && \
    sam delete --stack-name drive-deposits-event-rules-dev --region us-west-2

# event source with bus
deployed-delete-drive-deposits-event-bus: deployed-delete-drive-deposits-dynamodb-queries deployed-delete-drive-deposits-event-rules
    echo "deployed delete DriveDepositsEventBus" && \
    cd drive-deposits-event-source && \
    sam delete --stack-name drive-deposits-event-bus-dev --region us-west-2

# clean
clean-build-drive-deposit-event-bus:
    echo "cleaning build drive-deposits-event-bus" && \
    rm -rf ./drive-deposits-event-source/.aws-sam || true

clean-build-drive-deposit-event-rules: clean-build-drive-deposit-event-bus
    echo "cleaning build drive-deposit-event-rules" && \
    rm -rf ./drive-deposits-logs-lambda-target/.aws-sam || true
    rm -rf ./drive-deposits-logs-lambda-target/target || true

# localstack; can be started in Docker desktop terminal ( actual aws deploy counterpart above)
localstack-build:
    # Build the Docker image using the custom Dockerfile name
    LOCALSTACK_DEBUG=1 docker build -f Dockerfile.localstack -t custom-localstack .

localstack-start: localstack-build
    # Run a container from the built image, mapping the ports
    docker run -p 4566:4566 -p 4510-4559:4510-4559 \
            -v ${LOCALSTACK_VOLUME_DIR:-./volume}:/var/lib/localstack \
            -v /var/run/docker.sock:/var/run/docker.sock \
            --name localstack-container \
            custom-localstack

localstack-start-detached: localstack-build
    # Run a container from the built image, mapping the ports
    docker run -d -p 4566:4566 -p 4510-4559:4510-4559 \
            -v ${LOCALSTACK_VOLUME_DIR:-./volume}:/var/lib/localstack \
            -v /var/run/docker.sock:/var/run/docker.sock \
            --name localstack-container \
            custom-localstack

localstack-logs:
    # View logs of the LocalStack container
    docker logs localstack-container

localstack-stop:
    docker stop localstack-container && \
    docker rm localstack-container

# after localstack started

# in localstack event source with bus
localstack-build-drive-deposits-event-bus:
    echo "localstack building before deploy DriveDepositsEventBus" && \
    cd drive-deposits-event-source && \
    samlocal build --template-file template.yaml

# to not error out if samlocal deploy --config-env dev already exists
localstack-deploy-drive-deposits-event-bus: localstack-build-drive-deposits-event-bus
    echo "localstack deploy DriveDepositsEventBus" && \
    cd drive-deposits-event-source && \
    samlocal deploy --no-confirm-changeset --config-env dev || true

# in localstack event target with rule
localstack-build-drive-deposits-event-rules:
    echo "localstack building before deploy rules" && \
    cd drive-deposits-logs-lambda-target && \
    samlocal build --beta-features --template-file template.yaml

localstack-deploy-drive-deposits-event-rules: localstack-deploy-drive-deposits-event-bus localstack-build-drive-deposits-event-rules
    echo "localstack deploy rules"
    cd drive-deposits-logs-lambda-target && \
    samlocal deploy --no-confirm-changeset --config-env dev --parameter-overrides UseLocalstack="true" Environment="dev"

# only this receipe for localstack
localstack-deploy-drive-deposits-event-rules-only:
    echo "localstack deploy rules"
    cd drive-deposits-logs-lambda-target && \
    samlocal deploy --no-confirm-changeset --config-env dev --parameter-overrides UseLocalstack="true" Environment="dev"

awslocal-invoke-drive-deposits-event-rules-lambda:
    echo "invoking lambda function" && \
    cd drive-deposits-logs-lambda-target && \
    awslocal lambda invoke --invocation-type=Event --function-name drive_deposits_by_level_lambda_writer_dev --payload file://data/drive-deposits-event-banks-level.json --cli-binary-format raw-in-base64-out out.json

awslocal-list-dynamodb-tables:
    # --endpoint-url https://localhost.localstack.cloud:4566
    # dynamodb-proxy has dynamodb_endpoint_url = https://localhost.localstack.cloud:4566
    awslocal dynamodb list-tables --profile dynamodb-proxy

# localstack run
localstack-run-drive-deposits-check-cmd-valid-send-events:
    USE_LOCALSTACK="true" SEND_CAL_EVENTS="true" cargo ddcheck -- drive-deposits-rest-gateway-server/data/portfolio_request_valid.json

localstack-run-drive-deposits-check-cmd-valid-send-events-lesser-amount-investments:
    USE_LOCALSTACK="true" SEND_CAL_EVENTS="true" cargo ddcheck -- drive-deposits-rest-gateway-server/data/portfolio_request_valid_lesser_amount_investments.json

localstack-run-drive-deposits-check-cmd-valid-send-events-greater-amount-investments:
    USE_LOCALSTACK="true" SEND_CAL_EVENTS="true" cargo ddcheck -- drive-deposits-rest-gateway-server/data/portfolio_request_valid_greater_amount_investments.json

# running grpc server to send event to localstack just in case needed
localstack-run-drive-deposits-grpc-server:
    USE_LOCALSTACK="true" SEND_CAL_EVENTS="true" cargo run --package drive-deposits-grpc-server --bin drive-deposits-grpc-server

# localstack watch

# .cargo/config.toml has USE_LOCALSTACK = "false" so we can overrride it here as needed
localstack-watch-run-drive-deposits-check-cmd-valid-send-events:
    USE_LOCALSTACK="true" SEND_CAL_EVENTS="true" cargo watch -x "ddcheck -- drive-deposits-rest-gateway-server/data/portfolio_request_valid.json"

# localstack logs from samlocal
localstack-logs-drive-deposits-event-rules-lambda:
    echo "localstack logs" && \
    cd drive-deposits-logs-lambda-target && \
    samlocal logs --stack-name drive-deposits-event-rules-dev --name DriveDepositsByLevelLambdaFunction -t

# localstack clean automatically happens on shutodwn since localstack by default is ephemeral
localstack-clean-build-drive-deposit-event-rules: clean-build-drive-deposit-event-rules
    echo "localstack cleaned build drive-deposit-event-rules"

# rest gateway server curl POST commands to send calculation events to target through RESRT and gRPC servers  -- same as in request.http at drive-deposits-rest-gateway-server/request.http
# however had to exclude br since in curl not directly supported in curl verison 8.7.1
# the request.http supports it so alternatively that can be used for br brotli compression
# Define variables
# for k8s with ingress

rest_gateway_server_host := "http://api.drivedeposits.local"

#rest_gateway_server_host := "http://localhost:3000"

token := "Bearer token"

# Recipe for the POST request with root path
post-root:
    cd drive-deposits-rest-gateway-server && \
    curl -X POST {{ rest_gateway_server_host }}/ \
        -H "Content-Type: application/json" \
        -H "Authorization: {{ token }}" \
        -H "Accept-Encoding: gzip, deflate" \
        --data @./data/portfolio_request_valid.json \
        --compressed

# Recipe for the POST request with correct API path
post-calculate-portfolio-valid:
    cd drive-deposits-rest-gateway-server && \
    curl -X POST {{ rest_gateway_server_host }}/api/drive-deposits/calculate-portfolio \
        -H "Content-Type: application/json" \
        -H "Authorization: {{ token }}" \
        -H "Accept-Encoding: gzip, deflate" \
        --data @./data/portfolio_request_valid.json \
        --compressed \
        | jq

post-calculate-portfolio-valid-lesser-amount:
    cd drive-deposits-rest-gateway-server && \
    curl -X POST {{ rest_gateway_server_host }}/api/drive-deposits/calculate-portfolio \
        -H "Content-Type: application/json" \
        -H "Authorization: {{ token }}" \
        -H "Accept-Encoding: gzip, deflate" \
        --data @./data/portfolio_request_valid_lesser_amount_investments.json \
        --compressed \
        | jq

post-calculate-portfolio-valid-greater-amount:
    cd drive-deposits-rest-gateway-server && \
    curl -X POST {{ rest_gateway_server_host }}/api/drive-deposits/calculate-portfolio \
        -H "Content-Type: application/json" \
        -H "Authorization: {{ token }}" \
        -H "Accept-Encoding: gzip, deflate" \
        --data @./data/portfolio_request_valid_greater_amount_investments.json \
        --compressed \
        | jq

# Recipe for the invalid decimal request
post-calculate-portfolio-invalid-decimal:
    cd drive-deposits-rest-gateway-server && \
    curl -X POST {{ rest_gateway_server_host }}/api/drive-deposits/calculate-portfolio \
        -H "Content-Type: application/json" \
        -H "Authorization: {{ token }}" \
        -H "Accept-Encoding: gzip, deflate" \
        --data @./data/portfolio_request_invalid_decimal.json \
        --compressed

# Recipe for the invalid period unit, account type, decimal, and bank tz start date request
post-calculate-portfolio-invalid-period-unit-account-type-decimal-bank-tz-start-date:
    cd drive-deposits-rest-gateway-server && \
    curl --include -X POST {{ rest_gateway_server_host }}/api/drive-deposits/calculate-portfolio \
        -H "Content-Type: application/json" \
        -H "Authorization: {{ token }}" \
        -H "Accept-Encoding: gzip, deflate" \
        --data @./data/portfolio_request_invalid_period_unit_account_type_decimal_bank_tz_start_date.json \
        --compressed

# Recipe for the invalid JSON structure request
post-calculate-portfolio-invalid-json-structure:
    cd drive-deposits-rest-gateway-server && \
    curl -X POST {{ rest_gateway_server_host }}/api/drive-deposits/calculate-portfolio \
        -H "Content-Type: application/json" \
        -H "Authorization: {{ token }}" \
        -H "Accept-Encoding: gzip, deflate" \
        --data @./data/portfolio_request_invalid_json_structure.json \
        --compressed

# Recipe for the valid 90 days delta period request
post-calculate-portfolio-valid-90-days-delta-period:
    cd drive-deposits-rest-gateway-server && \
    curl -X POST {{ rest_gateway_server_host }}/api/drive-deposits/calculate-portfolio \
        -H "Content-Type: application/json" \
        -H "Authorization: {{ token }}" \
        -H "Accept-Encoding: gzip, deflate" \
        --data @./data/portfolio_request_valid_90_days_delta_period.json \
        --compressed \
        | jq

# Recipe for the valid 6 months delta period request
post-calculate-portfolio-valid-6-months-delta-period:
    cd drive-deposits-rest-gateway-server && \
    curl -X POST {{ rest_gateway_server_host }}/api/drive-deposits/calculate-portfolio \
        -H "Content-Type: application/json" \
        -H "Authorization: {{ token }}" \
        -H "Accept-Encoding: gzip, deflate" \
        --data @./data/portfolio_request_valid_6_months_delta_period.json \
        --compressed \
        | jq

########################
# lambda reader for dynamodb queries per writes by lambda writer to dynamodb in lambda target
########################
# cargo lambda for pure local development start without even localstack container
# if needed for cargo lambda without going through sam for just lambda function

# cargo lambda build --release
cargo-lambda-build-drive-deposits-lambda-dynamodb-reader:
    echo "building lambda reader function" && \
    cd drive-deposits-lambda-dynamodb-reader && \
    cargo lambda build

cargo-lambda-build-watching-drive-deposits-lambda-dynamodb-reader:
    USE_LOCALSTACK="true" echo "build watching lambda reader function" && \
    cd drive-deposits-lambda-dynamodb-reader && \
    cargo watch -x "lambda build"

# RUST_LOG=by_level_lambda_writer=debug not needed as added in .cargo/config.toml
# manually since localstack community version does not support apigwv2
# export DRIVE_DEPOSITS_TABLE_NAME=drive-deposits-event-rules-dev-DriveDepositsTable-62d90558
# export USE_LOCALSTACK="true"
# echo $DRIVE_DEPOSITS_TABLE_NAME

# echo $USE_LOCALSTACK
cargo-lambda-watch-drive-deposits-lambda-dynamodb-reader-localstack:
    echo "watching lambda reader function" && \
    cd drive-deposits-lambda-dynamodb-reader && \
    export DRIVE_DEPOSITS_TABLE_NAME={{ localstack_drive_deposit_table_name }} && \
    export USE_LOCALSTACK="true" && \
    echo $DRIVE_DEPOSITS_TABLE_NAME && \
    echo $USE_LOCALSTACK && \
    cargo lambda watch

# cargo lambda invoke by_level_lambda_reader --data-file data/apigw-event-request-query-for-portfolios.json
# based on cargo lambda invoke by_level_lambda_reader --data-example apigw-request --skip-cache
# alternatelively directly invoking lambda in postman quivalent to cargo lambda invoke:required special urls -- {{cargo_lambda_url}}/lambda-url/by_level_lambda_reader/by-level-for-portfolios/delta-growth where

# {{cargo_lambda_url}} is the url http://[::]:9000
cargo-lambda-invoke-drive-deposits-lambda-dynamodb-reader-apigw-event-query-for-portfolios-delta-growth:
    echo "invoking lambda reader function" && \
    cd drive-deposits-lambda-dynamodb-reader && \
    cargo lambda invoke by_level_lambda_reader --data-file data/apigw-event-request-query-for-portfolios-delta-growth.json

cargo-lambda-invoke-drive-deposits-lambda-dynamodb-reader-apigw-event-query-for-banks-delta-growth:
    echo "invoking lambda reader function" && \
    cd drive-deposits-lambda-dynamodb-reader && \
    cargo lambda invoke by_level_lambda_reader --data-file data/apigw-event-request-query-for-banks-delta-growth.json

cargo-lambda-invoke-drive-deposits-lambda-dynamodb-reader-apigw-event-query-for-deposits-delta-growth:
    echo "invoking lambda reader function" && \
    cd drive-deposits-lambda-dynamodb-reader && \
    cargo lambda invoke by_level_lambda_reader --data-file data/apigw-event-request-query-for-deposits-delta-growth.json

cargo-lambda-invoke-drive-deposits-lambda-dynamodb-reader-apigw-event-query-for-deposits-maturity-date:
    echo "invoking lambda reader function" && \
    cd drive-deposits-lambda-dynamodb-reader && \
    cargo lambda invoke by_level_lambda_reader --data-file data/apigw-event-request-query-for-deposits-maturity-date.json

# after full deploy and ingestion writes are completed
# dynamodb queries based on designed persistence in drive-deposits-logs-lambda-target
# in sequence of usage the recipes below

# dynamodb reader for queries
validate-drive-deposits-dynamodb-queries:
    echo "validating before building DriveDepositsByLevelLambdaReaderFunction" && \
    cd drive-deposits-lambda-dynamodb-reader && \
    sam validate --lint --template-file template.yaml

# aws guided

# if needed guided; usually needed to create samconfig.yml initially that can be modifed later
deploy-guided-drive-deposits-dynamodb-queries:
    echo "deploy guided event rules" && \
    cd drive-deposits-lambda-dynamodb-reader && \
    sam deploy --guided

# API Gateway v2 is not supported by localstack community edition...have to be on pro version
# here for sanity check for any other issues
# onto localstack guided

# initially manual confirmation
localstack-deploy-guided-drive-deposits-dynamodb-queries:
    echo "deploy guided event rules" && \
    cd drive-deposits-lambda-dynamodb-reader && \
    samlocal deploy --guided

localstack-build-drive-deposits-dynamodb-queries:
    echo "localstack building before deploy queries" && \
    cd drive-deposits-lambda-dynamodb-reader && \
    samlocal build --beta-features --template-file template.yaml

# note localstack community version does not support apigwv2
# use only with pro version of localstack

# error - To find out if AWS::ApiGatewayV2::Api is supported in LocalStack Pro, please check out our docs at https://docs.localstack.cloud/user-guide/aws/cloudformation/#resources-pro--enterprise-edition
localstack-deploy-drive-deposits-dynamodb-queries: localstack-deploy-drive-deposits-event-rules localstack-build-drive-deposits-dynamodb-queries
    echo "localstack deploy DriveDepositsByLevelLambdaReaderFunction" && \
    cd drive-deposits-lambda-dynamodb-reader && \
    samlocal deploy --no-confirm-changeset --config-env dev --parameter-overrides UseLocalstack="true" Environment="dev" DriveDepositsTableName="drive-deposits-event-rules-dev-DRIVE-DEPOSITS-TABLE-NAME" || true

# only this receipe for localstack

# note localstack community version does not support apigwv2
localstack-deploy-drive-deposits-dynamodb-queries-only:
    echo "localstack deploy DriveDepositsByLevelLambdaReaderFunction" && \
    cd drive-deposits-lambda-dynamodb-reader && \
    samlocal deploy --no-confirm-changeset --config-env dev --parameter-overrides UseLocalstack="true" Environment="dev" DriveDepositsTableName="drive-deposits-event-rules-dev-DRIVE-DEPOSITS-TABLE-NAME"

# back to aws
build-drive-deposits-dynamodb-queries: validate-drive-deposits-dynamodb-queries
    echo "building before deploy DriveDepositsByLevelLambdaReaderFunction" && \
    cd drive-deposits-lambda-dynamodb-reader && \
    sam build --beta-features --template-file template.yaml

deploy-drive-deposits-dynamodb-queries: deploy-drive-deposits-event-rules build-drive-deposits-dynamodb-queries
    echo "deploy event DriveDepositsByLevelLambdaReaderFunction"
    cd drive-deposits-lambda-dynamodb-reader && \
    sam deploy --no-confirm-changeset --config-env dev --parameter-overrides UseLocalstack="false" Environment="dev" DriveDepositsTableName="drive-deposits-event-rules-dev-DRIVE-DEPOSITS-TABLE-NAME"

deploy-drive-deposits-dynamodb-queries-only: build-drive-deposits-dynamodb-queries
    echo "deploy event DriveDepositsByLevelLambdaReaderFunction"
    cd drive-deposits-lambda-dynamodb-reader && \
    sam deploy --no-confirm-changeset --config-env dev --parameter-overrides UseLocalstack="false" Environment="dev" DriveDepositsTableName="drive-deposits-event-rules-dev-DRIVE-DEPOSITS-TABLE-NAME"

logs-drive-deposits-dynamodb-queries-lambda:
    echo "localstack logs" && \
    cd drive-deposits-logs-lambda-target && \
    sam logs --stack-name drive-deposits-dynamodb-queries --name DriveDepositsByLevelLambdaReaderFunction -t

deployed-delete-drive-deposits-dynamodb-queries:
    echo "deployed delete DriveDepositsByLevelLambdaReaderFunction" && \
    cd drive-deposits-lambda-dynamodb-reader && \
    sam delete --stack-name drive-deposits-dynamodb-queries --region us-west-2

# clean
clean-build-drive-deposits-dynamodb-queries:
    echo "drive-deposits-dynamodb-queries" && \
    rm -rf ./drive-deposits-lambda-dynamodb-reader/.aws-sam || true
    rm -rf ./drive-deposits-lambda-dynamodb-reader/target || true

# rest lambda query curl GET commands
# Successfully configured custom domain for API Gateway instead of Invoke URL https://6owusuo4b1.execute-api.us-west-2.amazonaws.com!

aws_api_gateway_host := "https://api-queries.drivedeposits.drinnovations.us"

get-query-by-level-portfolios-delta-growth:
    cd drive-deposits-lambda-dynamodb-reader && \
    curl '{{ aws_api_gateway_host }}/by-level-for-portfolios/delta-growth?order=desc&top_k=3' \
        | jq

aws_portfolio_uuid := "1913a98b-9a20-4a10-b2a7-c31295497c0a"

get-query-by-level-for-banks-delta-growth:
    cd drive-deposits-lambda-dynamodb-reader && \
    curl '{{ aws_api_gateway_host }}/portfolios/{{ aws_portfolio_uuid }}/by-level-for-banks/delta-growth?order=desc&top_k=10' \
        | jq

get-query-by-level-for-deposits-delta-growth:
    cd drive-deposits-lambda-dynamodb-reader && \
    curl '{{ aws_api_gateway_host }}/portfolios/{{ aws_portfolio_uuid }}/by-level-for-deposits/delta-growth?order=desc&top_k=10' \
        | jq

get-query-by-level-for-deposits-maturity-date:
    cd drive-deposits-lambda-dynamodb-reader && \
    curl '{{ aws_api_gateway_host }}/portfolios/{{ aws_portfolio_uuid }}/by-level-for-deposits/maturity-date?order=asc&top_k=3' \
        | jq

# docker compose for local

# for server-based grpc and rest servers
compose-build-grpc-server:
    docker compose --progress=plain -f compose.grpc.yaml build

compose-build-rest-server:
    docker compose --progress=plain -f compose.rest.yaml build

compose-up-grpc-server: compose-build-grpc-server
    docker compose --progress=plain -f compose.grpc.yaml up

compose-up-rest-server: compose-build-rest-server
    docker compose --progress=plain -f compose.rest.yaml up

compose-build-grpc-server-no-cache:
    docker compose --progress=plain -f compose.grpc.yaml build --no-cache

compose-build-rest-server-no-cache:
    docker compose --progress=plain -f compose.rest.yaml build --no-cache

compose-up-grpc-server-no-cache: compose-build-grpc-server-no-cache
    docker compose --progress=plain -f compose.grpc.yaml up

compose-up-rest-server-no-cache: compose-build-rest-server-no-cache
    docker compose --progress=plain -f compose.rest.yaml up

docker-prune:
    docker system prune -a

# k8s for local -- uses local images
k8s-grpc-server:
    docker build -t k8s-drive-deposits-grpc-server:latest -f Dockerfile.grpc . && \
    kubectl apply -f grpc-server.yaml

k8s-grpc-server-delete:
    kubectl delete -f grpc-server.yaml

k8s-rest-server:
    docker build -t k8s-drive-deposits-rest-gateway-server:latest -f Dockerfile.rest.gateway . && \
    kubectl apply -f rest-server.yaml

k8s-rest-server-delete:
    kubectl delete -f rest-server.yaml

# for git release tags
git-tag-add-local TAG:
    git tag -a {{ TAG }} -m "Release version {{ TAG }}"

git-tag-add-force-local TAG:
    git tag -a {{ TAG }} -f -m "Release version {{ TAG }}"

git-tag-add-remote TAG:
    git push origin refs/tags/{{ TAG }}

git-tag-add-force-remote TAG:
    git push origin refs/tags/{{ TAG }} --force

git-tag-delete-local TAG:
    git tag -d {{ TAG }}

git-tag-delete-remote TAG:
    git push origin :refs/tags/{{ TAG }}

git-tag-list-verbose:
    git tag -n

git-tag-show-commit TAG:
    @echo "Commit for tag {{ TAG }}:"
    @git show-ref --tags {{ TAG }} | cut -d ' ' -f 1 | xargs git show --no-patch --pretty='format:Commit SHA: %H%nCommit Message: %s'
