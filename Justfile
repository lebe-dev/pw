version := `cat Cargo.toml | grep version | head -1 | cut -d " " -f 3 | tr -d "\""`
image := "tinyops/pw"

init:
  rustup component add clippy
  cargo install cargo-llvm-cov

test-image-build:
  docker build --progress=plain -t app:dev .

run-backend:
  cargo run

run-frontend:
  cd frontend && yarn && npm run dev -- --port=4200

run-dev-image:
  docker compose -f docker-compose-dev.yml up -d

stop-dev-image:
  docker compose -f docker-compose-dev.yml down

lint:
  cargo fmt -- --check
  cargo clippy -- -D warnings
  cd frontend && yarn lint

test:
  cd frontend && yarn test run
  cargo test

test-all: test

build: lint
  cargo test --bin server
  cargo test --lib
  cargo test --test '*'

build-release-image: test-all
  docker build --progress=plain --platform=linux/amd64 -t {{image}}:{{version}} .

build-chart:
  helm package helm-chart/

trivy:
  trivy image --severity HIGH,CRITICAL {{image}}:{{version}}

release: build-release-image
  docker push {{image}}:{{version}}
  helm package helm-chart/
