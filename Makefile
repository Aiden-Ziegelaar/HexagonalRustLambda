fmt-rust:
	cargo fmt

fmt-terraform:
	tflint --init
	tflint --chdir=infra

build-debug:
	cargo lambda build --arm64

build-production:
	cargo lambda build --arm64 --release

npm-install:
	pushd test-integration && npm install && popd
	pushd test-artillery && npm install && popd

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
	terraform -chdir=infra destroy -auto-approve

build-deploy: build-production deploy

build-deploy-x86:
	cargo lambda build --x86-64 --release
	terraform -chdir=infra apply -var="architectures=[\"x86_64\"]" -auto-approve

build-deploy-test: test-rust build-deploy test-int

build-deploy-test-x86: test-rust build-deploy-x86 test-int

build-deploy-test-destroy: build-deploy-test destroy

build-deploy-test-destroy-x86: build-deploy-test-x86 destroy

load-test:
	cd test && npm run load-test