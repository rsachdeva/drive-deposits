## DriveDeposits: A Robust Financial Microservices System with Delta Growth Analysis, Powered by a Scalable Rust Backend.

[![CI Pipeline](https://github.com/rsachdeva/drive-deposits/actions/workflows/rust.yml/badge.svg)](https://github.com/rsachdeva/drive-deposits/actions/workflows/rust.yml)

### Server-based & Serverless: Utilizes request-response blocking and sends events using async Rust for non-blocking, event-driven temporal decoupling, supporting both synchronous and asynchronous operations.

<img src="DriveDeposits-Rust.png" alt="DriveDeposits" width="256" height="256" style="border-radius: 10%; vertical-align: middle;">

## Table of Contents

- [DriveDeposits System Design: Integrated Microservices Architecture](#drivedeposits-system-design-integrated-microservices-architecture)
- [Domain Driven Terminology](#domain-driven-terminology)
- [DriveDeposits: Architectural Pillars](#drivedeposits-architectural-pillars)
- [Synchronous Microservices Components](#synchronous-microservices-components)
- [Asynchronous Microservices Components](#asynchronous-microservices-components)
- [Bridging Synchronous and Asynchronous Components In DriveDeposits Microservices](#bridging-synchronous-and-asynchronous-components-in-drivedeposits-microservices)
- [Deployment of Microservices](#deployment-of-microservices)
- [Hybrid Integration Testing Tool](#hybrid-integration-testing-tool)
- [Running Tests](#running-tests)
    - [Integration tests](#integration-tests)
    - [Unit and Integration tests](#unit-and-integration-tests)
    - [End-to-End tests](#end-to-end-tests)
- [Data population](#data-population)
- [Querying with custom domain](#querying-with-custom-domain)
- [Development Tool: cargo lambda](#development-tool-cargo-lambda)
- [Development Tool: LocalStack](#development-tool-localstack)
- [Clean And Build](#clean-and-build)
- [Configurations for DriveDeposits](#configurations-for-drivedeposits)
- [Member crates in workspace](#member-crates-in-workspace)

### DriveDeposits System Design: Integrated Microservices Architecture

![DriveDeposits Design](DriveDeposits.drawio.svg)

[Back to Table of Contents](#table-of-contents)

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
  this [PortfolioRequest](drive-deposits-rest-gateway-server/data/portfolio_request_valid.json) example:

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
      See [PortfolioResponse](drive-deposits-rest-gateway-server/data/portfolio_response_for_valid.json) at the Deposit
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
  retrieving the top 'k' items (where 'k' is the number defined by the user in the query) in ascending or descending
  order. For example:

```curl
curl '{{aws_api_gateway_host}}/by-level-for-portfolios/delta-growth?order=asc&top_k=10' \
    | jq
```

```curl
curl '{{aws_api_gateway_host}}/portfolios/{{aws_portfolio_uuid}}/by-level-for-banks/delta-growth/?order=asc&top_k=10' \
        | jq
```

```curl
curl '{{aws_api_gateway_host}}/portfolios/{{aws_portfolio_uuid}}/by-level-for-deposits/maturity-date?order=asc&top_k=2' \
        | jq
```

* **Maturity date:** The date when a deposit or investment reaches its full value or the end of its term.
* **Sorting capabilities with top_k based on maturity date:** DriveDeposits allows sorting by maturity date,
  retrieving the top 'k' deposits (where 'k' is the number defined by the user in the query) in ascending or descending
  order. For
  example:

```curl
curl '{{aws_api_gateway_host}}/portfolios/{{aws_portfolio_uuid}}/by-level-for-deposits/maturity-date?order=asc&top_k=2' \
        | jq
```

[Back to Table of Contents](#table-of-contents)

### DriveDeposits: Architectural Pillars

* **Microservices Architecture:** DriveDeposits is built on a robust microservices foundation, combining both
  server-based and serverless components for maximum flexibility and scalability.
* **Server-based Microservices:** Utilizing gRPC (Tonic) and REST (Axum) for high-performance, inter-service
  communication and external API access.
* **Serverless Microservices:** Leveraging AWS Lambda for auto-scaling, event-driven processing of financial
  calculations and data management. This includes a sophisticated query system using AWS API Gateway + Lambda API
  allowing efficient querying of portfolio, bank, and deposit data at various levels.
* **Event-Driven Architecture:** Built on AWS EventBridge and EventBus, ensuring seamless communication and real-time
  data processing for dynamic financial calculations.
* **Rust-Powered Performance:** Benefiting from Rust's renowned speed, safety, and concurrency features for a highly
  efficient and reliable system.
* **AWS Rust SDK Integration:** Seamless interaction with AWS services, including Lambda runtime for serverless
  execution and DynamoDB for scalable data storage and retrieval.
* **Tokio-Powered Concurrency:** Efficient management of asynchronous tasks, including event processing with AWS
  EventBridge.
* **SAM Deployment:** Serverless components deployed using AWS Serverless Application Model (SAM), streamlining the
  deployment of Lambda functions, DynamoDB, and EventBridge rules.

**DriveDeposits offers a powerful and flexible microservices solution for:**

* **Financial institutions** modernizing their calculation infrastructure.
* **FinTech companies** seeking a scalable, reliable platform for real-time financial data processing.
* **Developers** building high-performance, innovative financial applications.

**Experience the future of microservices-based financial calculations with DriveDeposits!**
Documentation for Drive Deposits is a work in progress. More details will be added.

[Back to Table of Contents](#table-of-contents)

### Synchronous Microservices Components

- Using ***Axum***:
    - Serverless Microservice: AWS Lambda Reader (routes for querying data)
        - With AWS API Gateway
    - Server-Based Microservice: REST Gateway

- Using ***Tonic***
    - Server-based Microservice: gRPC Server

[Back to Table of Contents](#table-of-contents)

### Asynchronous Microservices Components

- using ***Tokio***
    - Asynchronous Tasks to Send Events to EventBridge

- Using the ***AWS SDK for Rust*** for:
    - Serverless Microservice: AWS Lambda Writer
    - AWS DynamoDB Integration
    - AWS EventBridge Integration

[Back to Table of Contents](#table-of-contents)

### Bridging Synchronous and Asynchronous Components In DriveDeposits Microservices

DriveDeposits is a cutting-edge financial calculation platform built on a robust microservices architecture that
combines synchronous and asynchronous components. The synchronous gRPC server microservice utilizes Tokio to
asynchronously send events to the EventBridge service, seamlessly integrating with the asynchronous microservices
components of the system.

The gRPC server microservice spawns asynchronous Tokio tasks to send events to EventBridge, effectively bridging the
synchronous and asynchronous parts of the architecture.

The Lambda Reader with Axum Endpoints component serves as a serverless microservice API built with AWS Lambda and the
Axum web framework in Rust. This microservice provides a set of endpoints to query and retrieve financial data from a
DynamoDB table. The endpoints are designed to fetch data at different levels of granularity, such as portfolios, banks,
and deposits, based on specific criteria like delta growth or maturity date.

The main functionalities of this microservice include:

- **Portfolio Level Endpoint**: Retrieves a list of portfolios based on the delta growth criteria.
- **Bank Level Endpoint**: Fetches a list of banks for a given portfolio UUID, sorted by the delta growth criteria.
- **Deposit Level Endpoints**:
    - **Delta Growth**: Retrieves a list of deposits for a given portfolio UUID, sorted by the delta growth criteria.
    - **Maturity Date**: Fetches a list of deposits for a given portfolio UUID, sorted by the maturity date.

This serverless microservice interacts with an AWS DynamoDB table to read and query the required data. It utilizes the
`aws-sdk-rust` crate to communicate with the DynamoDB service. The Axum web framework is used to define the API routes
and handle HTTP requests and responses, making it a highly efficient and scalable component of the DriveDeposits
microservices ecosystem.

[Back to Table of Contents](#table-of-contents)

### Deployment of Microservices

- **Serverless**: AWS using SAM

This project uses SAM (Serverless Application Model) for deploying the following AWS resources that support our
microservices architecture:

    * EventBridge
    * EventBus
    * Targets (including CloudWatch Log group)
    * Lambda functions (for serverless microservices)
    * DynamoDB table

The SAM-based deployment streamlines the process of setting up and managing these AWS services for the DriveDeposits
microservices application.

This project also utilizes the [Justfile](https://github.com/casey/just) for managing project-level recipes and
microservices deployment tasks.

#### Deploy all serverless microservices and AWS resources

`just deploy-drive-deposits-dynamodb-queries`

This command calls dependent recipes to deploy the entire microservices ecosystem, including the event bus, event rules
with lambda targets, and lambda functions for queries.

#### Delete all serverless microservices and AWS resources

`just deployed-delete-drive-deposits-event-bus`

This command calls dependent recipes to delete all deployed microservices and resources, including event rules, lambda
targets, event bus, and query-related resources.

#### Deploy event bridge with event rules and target lambda microservices components

`just deploy-drive-deposits-event-rules`

This command deploys the event-driven components of our microservices architecture, including EventBridge, EventBus, and
related resources.

#### Deploy Lambda Function with API Gateway and DynamoDB Reader microservices components

`just deploy-drive-deposits-dynamodb-queries-only`

#### Run the REST and gRPC servers for synchronous microservices with and without Docker

- **Server-based**:
    - **Natively** (without Docker)

      `just run-drive-deposits-grpc-server`

      `just run-drive-deposits-rest-grpc-gateway-server`

    - **Docker Compose**

      `just compose-up-grpc-server`

      `just compose-up-rest-server`

#### Test microservices integration

Send an HTTP request to the REST gateway server, which communicates with other microservices:

`just post-calculate-portfolio-valid`

This command tests the full microservices pipeline, from the REST gateway to the gRPC server, through to AWS EventBridge
and Lambda functions.

Follow up with the [Data population](#data-population) section to see how to query data across the microservices.

#### Test AWS Lambda microservice directly

`just aws-invoke-drive-deposits-event-rules-lambda`

[Back to Table of Contents](#table-of-contents)

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

[Back to Table of Contents](#table-of-contents)

### Running Tests

##### Integration tests

`just test-intg`

##### Unit and Integration tests

`just test`

##### End-to-End tests

`just test-e2e`

[Back to Table of Contents](#table-of-contents)

### Data population

#### AWS Populated with Basic Data for Queries Lambda

##### Using REST gateway and gRPC Servers: start

`just run-drive-deposits-grpc-server`

`just run-drive-deposits-rest-grpc-gateway-server`

##### send curl post requests to populate

`just post-calculate-portfolio-valid`

`just post-calculate-portfolio-valid-lesser-amount`

`just post-calculate-portfolio-valid-greater-amount`

##### Alternatively, Using check command without servers

`just run-drive-deposits-check-cmd-valid-send-events`

[Back to Table of Contents](#table-of-contents)

### Querying with custom domain

Successfully configured custom domain (https://api-queries.drivedeposits.drinnovations.us) for API Gateway so with some
existing data queries can be tried
by running just recipes for querying

Then you can perform the following from [justfile](justfile) for portfolios delta growth sorted in descending order with
top 3 items:

`just get-query-by-level-portfolios-delta-growth`

or directly click link to see some data in your browser or Postman like tool - with pretty print:

[Drive Deposits By Level For Portfolios API Query](https://api-queries.drivedeposits.drinnovations.us/by-level-for-portfolios/delta-growth?order=desc&top_k=3)

which actually calls

```curl
https://api-queries.drivedeposits.drinnovations.us/by-level-for-portfolios/delta-growth?order=desc&top_k=3
```

Update aws_portfolio_uuid per the portfolios response to populate
in [justfile](justfile). Adjusted `justfile` with a test portfolio UUID for querying.

`just get-query-by-level-portfolios-delta-growth`

`just get-query-by-level-for-deposits-delta-growth`

`just get-query-by-level-for-deposits-maturity-date`

[Back to Table of Contents](#table-of-contents)

### Development Tool: cargo lambda

Following is convenience so that in development can iterate faster when skipping sam resources:

for only build watching lambda without using sam
`just cargo-lambda-build-watching-drive-deposits-event-rules-lambda`

this is different from cargo lambda watch used with invoke -- The watch subcommand emulates the AWS Lambda control plane
API. The function is not compiled until the first time that you try to execute it.
`just cargo-lambda-watch-drive-deposits-event-rules-lambda`

#### command for cargo lambda invoke check directly

`just cargo-lambda-invoke-drive-deposits-event-rules-lambda`

[Back to Table of Contents](#table-of-contents)

### Development Tool: LocalStack

Localstack, being an ephemeral service, can be started and stopped as needed. This means that unless the state is
persisted, LocalStack provides a clean slate every time it's restarted. This feature can expedite the development and
deployment process as it eliminates the need to manually delete resources.

Following is convenience so that in development can iterate faster:

`just localstack-start`

Should see "Ready." -- There is a Terminal now in Docker Desktop itself so that is a good place to run this command.

#### deploy everything serverless in Localstack - aws deployment related commands for EventBridge, EventBus, Cloudwatch log groups and Lambda target function for writing to DynamoDB and Lambda DynamoDB reader

`just localstack-deploy-drive-deposits-dynamodb-queries`

#### LocalStack populated with data

Sanity check with LocalStack/ Working with queries lambda directly with cargo lambda and localstack

`just localstack-start`

`just localstack-deploy-drive-deposits-event-rules`

Populate with

`just localstack-run-drive-deposits-check-cmd-valid-send-events`

and then

`just localstack-run-drive-deposits-check-cmd-valid-send-events-lesser-amount-investments`

and then

`just localstack-run-drive-deposits-check-cmd-valid-send-events-greater-amount-investments`

and can also repeat these commands for more data

There is also a watch command that can be used to watch for changes in the code and automatically run the check command
`just localstack-watch-run-drive-deposits-check-cmd-valid-send-events`

For queries lambda, replace table name in localstack in
cargo-lambda-watch-drive-deposits-lambda-dynamodb-reader-localstack recipe

Then can use cargo-lambda with dynamodb in localstack already populated with data:
`just cargo-lambda-watch-drive-deposits-lambda-dynamodb-reader-localstack`

And for actual cargo lambda apigw events
`just cargo-lambda-invoke-drive-deposits-lambda-dynamodb-reader-apigw-event-query-for-portfolios`

Replace proper portfolio uuid

`just cargo-lambda-invoke-drive-deposits-lambda-dynamodb-reader-apigw-event-query-for-banks`

`just cargo-lambda-invoke-drive-deposits-lambda-dynamodb-reader-apigw-event-query-for-deposits`

##### command for awslocal localstack lambda invoke check directly

Following is convenience so that in development can iterate faster:

`just awslocal-invoke-drive-deposits-event-rules-lambda`

[Back to Table of Contents](#table-of-contents)

### Clean And Build

cargo clean is used but since there are lambda we have .aws-sam folders created by sam also that we have a clean

so we have
`just clean-with-lambdas`

for quick build that can be used after cleaning everything
`just build-with-lambdas`

[Back to Table of Contents](#table-of-contents)

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

[Back to Table of Contents](#table-of-contents)

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

[Back to Table of Contents](#table-of-contents)

[Copyright (c) 2024 Rohit Sachdeva](LICENSE)
