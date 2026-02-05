version := `cat Cargo.toml | grep version | head -1 | cut -d " " -f 3 | tr -d "\""`
chartVersion := `cat helm-chart/Chart.yaml | yq -r '.version'`
image := "tinyops/pw"
trivyReportFile := "docs/security/trivy-scan-report.txt"

init:
    rustup component add clippy
    cargo install cargo-llvm-cov

build-dev-image:
    docker build --progress=plain --platform=linux/amd64 .

format:
    cargo fmt

lint: format
    cargo clippy -- -D warnings
    cd frontend && yarn lint

test:
    cd frontend && yarn test run
    cargo test

build: lint && test
    cargo build

########################################
# DEV ENV
########################################

run-backend:
    cargo run

run-frontend:
    cd frontend && yarn && npm run dev -- --port=4200

start-dev-image:
    docker compose -f docker-compose-dev.yml up -d --build --force-recreate

stop-dev-image:
    docker compose -f docker-compose-dev.yml down

########################################
# HELM CHART
########################################

test-chart:
    helm template helm-chart/

build-chart: test-chart
    helm package helm-chart/ --app-version {{ version }}

release-chart: build-chart
    rm -rf helm-repo
    git clone git@github.com:tinyops-ru/tinyops-ru.github.io.git helm-repo
    bash -euo pipefail -c '\
        cd helm-repo && \
        cp ../pw-{{ chartVersion }}.tgz helm-charts/ && \
        helm repo index helm-charts/ && \
        if [ -z "$(git status --porcelain)" ]; then \
            echo "Chart pw-{{ chartVersion }} already published, skipping." && \
            exit 0; \
        fi && \
        git add helm-charts/ && \
        git commit -m "Add helm chart: pw-{{ chartVersion }}" && \
        git push'
    rm -rf helm-repo

########################################
# SECURITY
########################################

trivy:
    trivy image --severity HIGH,CRITICAL {{ image }}:{{ version }}

########################################
# RELEASE

# #######################################
build-release-image: lint && test
    docker build --progress=plain --platform=linux/amd64 -t {{ image }}:{{ version }} .

trivy-save-reports:
    trivy -v > {{ trivyReportFile }}
    trivy config Dockerfile >> {{ trivyReportFile }}
    trivy image --severity HIGH,CRITICAL {{ image }}:{{ version }} >> {{ trivyReportFile }}

release: build-release-image && release-chart
    docker push {{ image }}:{{ version }}
    just trivy-save-reports
