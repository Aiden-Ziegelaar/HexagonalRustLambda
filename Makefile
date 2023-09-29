fmt-rust:
	cargo fmt

fmt-terraform:
	tflint --init
	tflint --chdir=infra

build-debug:
	cargo lambda build --arm64

build-production:
	cargo lambda build --arm64 --release

test: test-rust test-integration

test-rust:
	cargo test

test-int:
	cd test-integration && npm test

init:
	terraform -chdir=infra init

plan:
	terraform -chdir=infra plan

deploy:
	terraform -chdir=infra apply -auto-approve

destroy:
	terraform -chdir=infra destroy

build-deploy: build-production deploy

build-deploy-x86:
	cargo lambda build --x86-64 --release
	terraform -chdir=infra apply -var="architectures=[\"x86_64\"]" -auto-approve

build-deploy-test: build-production deploy test-int

load-test:
	cd test && npm run load-test