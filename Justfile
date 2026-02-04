version := `cat Cargo.toml | grep version | head -1 | cut -d " " -f 3 | tr -d "\""`
image := "tinyops/pw"
trivyReportFile := "docs/security/trivy-scan-report.txt"

init:
  rustup component add clippy
  cargo install cargo-llvm-cov

build-dev-image:
  docker build --progress=plain --platform=linux/amd64 .

run-backend:
  cargo run

run-frontend:
  cd frontend && yarn && npm run dev -- --port=4200

start-dev-image:
  docker compose -f docker-compose-dev.yml up -d --build --force-recreate

stop-dev-image:
  docker compose -f docker-compose-dev.yml down

format:
  cargo fmt

lint: format
  cargo clippy -- -D warnings
  cd frontend && yarn lint

test:
  cd frontend && yarn test run
  cargo test

test-chart:
  helm template helm-chart/

build: lint && test
  cargo build

build-chart: test-chart
  helm package helm-chart/

trivy:
  trivy image --severity HIGH,CRITICAL {{image}}:{{version}}

########################################
# RELEASE
########################################
build-release-image: lint && test
  docker build --progress=plain --platform=linux/amd64 -t {{image}}:{{version}} .

trivy-save-reports:
  trivy -v > {{trivyReportFile}}
  trivy config Dockerfile >> {{trivyReportFile}}
  trivy image --severity HIGH,CRITICAL {{image}}:{{version}} >> {{trivyReportFile}}

release: build-release-image
  docker push {{image}}:{{version}}
  just build-chart
  just trivy-save-reports
