test:
	cargo test

fmt-rust:
	cargo fmt

fmt-terraform:
	tflint --init
	tflint --chdir=infra

build-debug:
	cargo lambda build --arm64

build-production:
	cargo lambda build --arm64 --release

init:
	terraform -chdir=infra init

plan:
	terraform -chdir=infra plan

deploy:
	terraform -chdir=infra apply

build-deploy: build-production deploy