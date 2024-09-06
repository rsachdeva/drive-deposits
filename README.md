## DriveDeposits: A Robust Financial System with Delta Growth Analysis, Powered by a Scalable Rust Backend (Synchronous & Asynchronous)

<img src="DriveDeposits-Rust.png" alt="DriveDeposits" width="256" height="256" style="border-radius: 10%; vertical-align: middle;">

## Table of Contents

- [Domain Driven Terminology](#domain-driven-terminology)
- [DriveDeposits: Architectural Pillars](#drivedeposits-architectural-pillars)
- [Synchronous Components](#synchronous-components)
- [Asynchronous Components](#asynchronous-components)
- [Bridging Synchronous and Asynchronous Components In DriveDeposits](#bridging-synchronous-and-asynchronous-components-in-drivedeposits)
- [AWS EventBridge, EventBus and Targets include Cloudwatch Log group and Lambda Deployment](#aws-eventbridge-eventbus-and-targets-include-cloudwatch-log-group-and-lambda-deployment)
- [Hybrid Integration Testing Tool](#hybrid-integration-testing-tool)
- [Development Tool: LocalStack](#development-tool-localstack)
- [Data population and Querying](#data-population-and-querying)
- [Development Tool: cargo lambda](#development-tool-cargo-lambda)
- [Clean](#clean)
- [Configurations for DriveDeposits](#configurations-for-drivedeposits)
- [Member crates in workspace](#member-crates-in-workspace)

### Domain Driven Terminology:

The following terms are consistently used throughout the DriveDeposits project, forming the core vocabulary of our
domain-specific language in both design and development:

* **Portfolio:** A collection of investments, such as stocks, bonds, and mutual funds, owned by an individual or an
  organization.
* **Bank:** A financial institution that accepts deposits for investment. In this context, "bank" refers to any
  financial institution.
* **Deposit:** A sum of money placed in a bank account or other investment vehicle.
* **Levels (Portfolio, Bank, Deposit):**  DriveDeposits allows query of data at different levels - portfolios, banks,
  or deposits - to make informed financial decisions.
* **Delta:**  Represents the growth over a user-specified period, as specified
  within the Portfolio
  Request. It
  measures investment
  performance at
  various levels (portfolios, banks, deposits). See
  this [PortfolioRequest](drive-deposits-rest-gateway-server/data/rest_request_valid.json) example:

  ```json
  {
      "new_delta": {
        "period": "1",
        "period_unit": "Month"
      }
  }
  ```

    * **Delta Growth:** The increase in value calculated in Portfolio Response. See JSON Path as
      outcome.delta.growth.
      This fluctuation is calculated at the portfolio, bank, and deposit levels.
      See [PortfolioResponse](drive-deposits-rest-gateway-server/data/rest_response_for_valid.json) at the Deposit
      Level for
      example:
      ```json
      {
        "uuid": "eb8ea161-c461-4b1a-8f7c-7b845ba5bcbc",
        "account": "1235N",
        "account_type": "BrokerageCertificateOfDeposit",
        "apy": "2.4",
        "years": "7",
        "outcome": {
          "delta": {
            "period": "1",
            "period_unit": "Month",
            "growth": "21.68"
          },
          "maturity": {
            "amount": "10990",
            "interest": "1846.32",
            "total": "12836.32"
          },
          "errors": []
        },
        "outcome_with_dates": {
          "start_date_in_bank_tz": "2024-02-16",
          "maturity_date_in_bank_tz": "2031-02-14",
          "errors": []
        }
      } 
      ```
      And at the Bank Level:
      ```json
      {
        "outcome": {
          "delta": {
            "period": "1",
            "period_unit": "Month",
            "growth": "246.16"
          },
          "maturity": {
            "amount": "71100",
            "interest": "7765.94",
            "total": "78865.94"
          },
          "errors": []
        }
      }
      ```
      And at the Portfolio Level:
      ```json
      {
        "outcome": {
          "delta": {
          "period": "1",
          "period_unit": "Month",
          "growth": "367.76"
          },
          "maturity": {
          "amount": "108580.50",
          "interest": "24462.92",
          "total": "133043.42"
          },
          "errors": []
          }
      }
      ```


* **Sorting Capabilities with Top K Based on Delta Growth:** DriveDeposits allows sorting based on delta growth,
  retrieving the top 'k' portfolios in ascending or descending order. For example:

```http
{{drive_deposits_lambda_reader}}/by-level-for-portfolios/delta-growth?&order=desc&top_k=3
```

```http
{{drive_deposits_lambda_reader}}/portfolios/{{aws_portfolio_uuid}}/by-level-for-banks/delta-growth?order=asc&top_k=9
```

* **Maturity date:** The date when a deposit or investment reaches its full value or the end of its term.
* **Sorting capabilities with top_k based on maturity date:** DriveDeposits allows sorting by maturity date,
  retrieving the top 'k' portfolios (where 'k' is a number defined by the user in the query) in ascending or descending
  order. For
  example:

```http
{{drive_deposits_lambda_reader}}/by-level-for-deposits/maturity-date?&order=desc&top_k=3
```

### DriveDeposits: Architectural Pillars

* **Event-Driven Architecture:** Built on AWS EventBridge and EventBus, DriveDeposits ensures seamless communication and
  real-time data processing for dynamic financial calculations.
* **Scalable and Reliable:** AWS Lambda scales automatically to handle fluctuating workloads, ensuring consistent
  performance and availability.
* **Rust-Powered Performance:** Built with Rust, DriveDeposits benefits from the language's renowned speed, safety, and
  concurrency, resulting in a highly efficient and reliable system.
* **AWS Rust SDK Integration:** DriveDeposits leverages the official AWS SDK for Rust to interact seamlessly with
  essential AWS services. This includes using the Lambda runtime for serverless function execution and DynamoDB for
  efficient and scalable data storage and retrieval.
* **High-Performance Computing:**  gRPC (Tonic) provides blazing-fast communication between services, while REST (Axum)
  offers a user-friendly API gateway for external integrations.
* **Seamless Integration:** DriveDeposits integrates seamlessly with essential AWS services like CloudWatch for
  monitoring based on EventBridge Rules.
* **Tokio-Powered Concurrency:** DriveDeposits leverages Tokio's powerful concurrency primitives to manage asynchronous
  tasks efficiently. This includes sending events to AWS EventBridge for further
  processing and analysis, ensuring a responsive and non-blocking system.
* **SAM Deployment:** The serverless components in this project are deployed using AWS Serverless Application Model (
  SAM). This includes AWS Lambda functions for writer and reader, as well as DynamoDB and EventBridge rules.

**DriveDeposits offers a powerful and flexible solution for:**

* **Financial institutions** looking to modernize their calculation infrastructure.
* **FinTech companies** seeking a scalable and reliable platform for real-time financial data processing.
* **Developers** building innovative financial applications that demand high performance and reliability.

**Experience the future of financial calculations with DriveDeposits!**
Documentation for Drive Deposits is work in progress. More details will be added.

### Synchronous Components

- AWS Serverless Lambda Reader (uses ***Axum*** for Routing)
    - AWS API Gateway
- gRPC (using ***Tonic***)
- REST (using ***Axum***)

### Asynchronous Components

Using the ***AWS SDK for Rust***

- AWS Serverless Lambda Writer
- DynamoDB
- EventBridge

### Bridging Synchronous and Asynchronous Components In DriveDeposits

DriveDeposits is a cutting-edge financial calculation platform built on a robust architecture that combines synchronous
and asynchronous components. The synchronous gRPC server utilizes Tokio to asynchronously send events to the EventBridge
service, seamlessly integrating with the asynchronous components of the system.
The gRPC server spawns asynchronous Tokio tasks to send events to
EventBridge, bridging the synchronous and asynchronous parts of the architecture.

Lambda Reader with Axum Endpoints component serves as a serverless API built with AWS Lambda and the Axum web framework
in Rust. It provides a set of endpoints to query and retrieve financial data from a DynamoDB table. The endpoints are
designed to fetch data at different levels of granularity, such as portfolios, banks, and deposits, based on specific
criteria like delta growth or maturity date.

The main functionalities include:

- **Portfolio Level Endpoint**: Retrieves a list of portfolios based on the delta growth criteria.
- **Bank Level Endpoint**: Fetches a list of banks for a given portfolio UUID, sorted by the delta growth criteria.
- **Deposit Level Endpoints**:
    - **Delta Growth**: Retrieves a list of deposits for a given portfolio UUID, sorted by the delta growth criteria.
    - **Maturity Date**: Fetches a list of deposits for a given portfolio UUID, sorted by the maturity date.

The Lambda function interacts with an AWS DynamoDB table to read and query the required data. It utilizes
the `aws-sdk-rust` crate to communicate with the DynamoDB service. The Axum web framework is used to define the API
routes and handle HTTP requests and responses.

### AWS EventBridge, EventBus and Targets include Cloudwatch Log group and Lambda Deployment

This project utilizes the [Justfile](https://github.com/casey/just) for managing project-level tasks.

#### deploy everything -- aws deployment related commands for EventBridge, EventBus, Cloudwatch log groups and Lambda target function for writing to DynamoDB and Lambda DynamoDB reader

`just deploy-drive-deposits-dynamodb-queries`

calls dependent recipes - deploys [event bus and then event rules with lambda target] and then lambda function for
queries

#### delete everything -- aws deployment related commands for EventBridge, EventBus, Cloudwatch log groups and Lambda target function for writing to DynamoDB and Lambda DynamoDB reader

`just deployed-delete-drive-deposits-event-bus`

calls dependent recipes - deletes [event rule with lambda target and then event bus] and then queries from dependent
recipes

#### create aws deployment related commands for EventBridge, EventBus, Cloudwatch log groups and Lambda Write DynamoDB function

`just deploy-drive-deposits-event-rules`

calls dependent recipe and also creates event bus from dependent recipe

#### AWS Deployment for Lambda Function with API Gateway and Lambda DynamoDB Reader

`just deploy-drive-deposits-dynamodb-queries-only`

#### Run REST gateway and gRPC Servers

`just run-drive-deposits-grpc-server`

`just run-drive-deposits-rest-grpc-gateway-server`

#### Send http request to rest gateway server

This sends the full Portfolio request body to the REST gateway server that sends the request to the gRPC server. The
gRPC server then performs calculations and sends calculation events to AWS EventBridge with targets for log groups, and
a Lambda function connected to DynamoDB.

`just post-calculate-portfolio-valid`

#### Command for AWS Lambda Invoke Check Directly

`just aws-invoke-drive-deposits-event-rules-lambda`

### Hybrid Integration Testing Tool

#### Efficient Command for Synchronous Calculation Flow and Asynchronous AWS Serverless Event Testing

The command drive-deposits-check-cmd is a powerful hybrid integration testing tool that bridges both synchronous and
asynchronous aspects of the system. It efficiently mimics the synchronous flow of REST and gRPC servers while
interacting with asynchronous EventBridge components, all without the need for full server deployment.
Key features:

* Performs identical type transformations as REST and gRPC servers
* Enables rapid calculation validation and event routing verification
* Sends calculations to EventBridge for comprehensive sanity testing
* Validates event routing to appropriate destinations (log groups, AWS Lambda functions, DynamoDB)
* Allows developers to verify end-to-end flow of calculations, event handling, and data persistence
  Execute the tool with:
  `just run-drive-deposits-check-cmd-valid-send-events`
  This streamlined approach significantly enhances development efficiency and system reliability testing.

###### Alias

.cargo/config.toml has alias for command line [drive-deposits-check-cmd](drive-deposits-check-cmd) so can be run using
`cargo ddcheck` For help see `cargo ddcheck -- --help`

### Development Tool: LocalStack

Localstack, being an ephemeral service, can be started and stopped as needed. This means that unless the state is
persisted, LocalStack provides a clean slate every time it's restarted. This feature can expedite the development and
deployment process as it eliminates the need to manually delete resources.

Following is convenience so that in development can iterate faster:

`just localstack-start`

Should see "Ready." -- There is a Terminal now in Docker Desktop itself so that is a good place to run this command.

#### deploy everything in localstack -- aws deployment related commands for EventBridge, EventBus, Cloudwatch log groups and Lambda target function for writing to DynamoDB and Lambda DynamoDB reader

`just localstack-deploy-drive-deposits-dynamodb-queries`

### Data population and Querying

#### AWS Populated with Basic Data for Queries Lambda

##### Using REST gateway and gRPC Servers: start

`just run-drive-deposits-grpc-server`

`just run-drive-deposits-rest-grpc-gateway-server`

##### send curl post requests to populate

`just post-calculate-portfolio-valid`

`just post-calculate-portfolio-valid-lesser-amount`

`just post-calculate-portfolio-valid-greater-amount`

##### Alternatively Using check command without servers

`just run-drive-deposits-check-cmd-valid-send-events`

##### For actual AWS lambda query request

`just get-query-by-portfolios-level`

#### LocalStack populated with data

Sanity check with LocalStack/ Working with queries lambda directly with cargo lambda and

`just localstack-start`

`just localstack-deploy-drive-deposits-event-rules`

Populate with
`just localstack-run-drive-deposits-check-cmd-valid-send-events`
and then
`just localstack-run-drive-deposits-check-cmd-valid-send-events-lesser-amount-investments`
and then
`just localstack-run-drive-deposits-check-cmd-valid-send-events-greater-amount-investments`
and can also repeat these commands

There is also a watch command that can be used to watch for changes in the code and automatically run the check command
`just localstack-watch-run-drive-deposits-check-cmd-valid-send-events`

For queries lambda; replace table name in localstack in
cargo-lambda-watch-drive-deposits-lambda-dynamodb-reader-localstack-for-dynamodb

Then can use cargo-lambda with dynamodb in localstack already populated with data:
`just cargo-lambda-watch-drive-deposits-lambda-dynamodb-reader-localstack-for-dynamodb`

And for actual cargo lambda apigw events
`just cargo-lambda-invoke-drive-deposits-lambda-dynamodb-reader-apigw-event-query-for-portfolios`

Replace proper portfolio uuid

`just cargo-lambda-invoke-drive-deposits-lambda-dynamodb-reader-apigw-event-query-for-banks`

`just cargo-lambda-invoke-drive-deposits-lambda-dynamodb-reader-apigw-event-query-for-deposits`

##### command for awslocal localstack lambda invoke check directly

Following is convenience so that in development can iterate faster:

`just awslocal-invoke-drive-deposits-event-rules-lambda`

### Development Tool: cargo lambda

Following is convenience so that in development can iterate faster when skipping sam resources:

for build watching lambda without using sam
`just cargo-lambda-build-watching-drive-deposits-event-rules-lambda`

this is different from cargo lambda watch used with invoke
`just cargo-lambda-watch-drive-deposits-event-rules-lambda`

#### command for cargo lambda invoke check directly

`just cargo-lambda-invoke-drive-deposits-event-rules-lambda`

### Clean

cargo clean is used but since there are lambda we have .aws-sam folders created by sam also that we have a clean

so we have
`just clean-with-lambdas`

for quick build only that can be used after cleaning
`just build-with-lambdas`

### Configurations for DriveDeposits

The project uses custom configurations defined in `.cargo/config.toml`:

- `SEND_CAL_EVENTS`: This environment variable is set to "true" by default in the config file. It can be overridden in
  specific commands as needed.
- `USE_LOCALSTACK`: This environment variable is set to "false" by default in the config file. It can be overridden for
  local development with LocalStack.
- Alias: The project includes an alias for the `drive-deposits-check-cmd`. It can be run using `cargo ddcheck`. For
  help, use `cargo ddcheck -- --help`.

These configurations allow for flexible development and testing environments, enabling easy switching between local and
AWS deployments.

### Member crates in workspace

See cargo workspace members:
[Cargo.toml](Cargo.toml)

#### Naming: why keeping prefix drive-deposits

Using the "drive-deposits-" prefix for crate names clearly distinguishes these as separate crates within
the workspace, not just modules within a single crate. This distinction is crucial for understanding the project
structure and for managing dependencies. It also allows for more flexibility in terms of versioning and publishing each
crate independently if needed. This naming convention effectively communicates the relationship between the crates while
maintaining their individual identities within the Rust ecosystem.

#### Summary of the Responsibilities for crates drive-deposits-logs-lambda-target and drive-deposits-lambda-dynamodb-reader in Workspace

###### drive-deposits-logs-lambda-target:

* Triggered by EventBridge
* Handles log groups based on event rules
* Writes data to DynamoDB
* Error handling when adding items in DynamoDB indicates the source of the error and the level context in which it
  occurred.
* Error logs can be seen in CloudWatch Logs

###### drive-deposits-lambda-dynamodb-reader:

* Responsible for querying data from DynamoDB
* Uses Axum in Lambda
* Exposed through an API Gateway
* Error handling when reading items from DynamoDB and creating Response for Query requests indicates the source of the
  error and the level context in which it occurred.
* Error logs can be seen in CloudWatch Logs

[Copyright (c) 2024 Rohit Sachdeva](LICENSE)
