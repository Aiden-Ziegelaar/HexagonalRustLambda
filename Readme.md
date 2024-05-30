# Hexagonal Architecture for Rust Lambdas

For a full breakdown of the methodology behind this project there is a blog post here:
TODO: Add Blog post when published

## Getting Started
What you'll need:
* [An AWS Account](https://aws.amazon.com/free/)\
* [Rust installed via rustup](https://rustup.rs/)
* [Cargo Lambda](https://www.cargo-lambda.info/guide/getting-started.html)
* [Terraform](https://www.terraform.io/)

## Makefile Commands
* `fmt-rust` runs cargo fmt on the project
* `fmt-terraform` runs tflint on the project
* `build-debug` creates a build of the app with debug symbols (this can be quiet large)
* `build-production` creates optimised productions builds of the app
* `npm-install` installs dependencies for integration and load tests
* `test` runs unit and integration tests
* `test-rust` runs unit tests
* `test-int` runs integration tests (Requires a deployed environment)
* `init` initialises terraform
* `plan` creates a plan using terraform
* `deploy` deploys resorces to AWS (requires a previous build)
* `destroy` destroys cloud resources via terraform destroy
* `build-deploy` builds app and deploys to AWS on ARM architecture
* `build-deploy-x86` same as `build-deploy` but on x86-64 architecture
* `build-deploy-test` runs unit tests, build app for production, deploys app, and runs integration tests
* `build-deploy-test-x86` same as `build-deploy-test` but on x86-64 architecture
* `build-deploy-test-destroy` same as `build-deploy-test` but with environment teardown at the end
* `build-deploy-test-destroy-x86` same as `build-deploy-test-destroy` but on x86-64 architecture
* `load-test` runs artillery tests against app