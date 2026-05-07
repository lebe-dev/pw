version := `cat Cargo.toml | grep version | head -1 | cut -d " " -f 3 | tr -d "\""`
chartName := `cat helm-chart/Chart.yaml | yq -r '.name'`
chartVersion := `cat helm-chart/Chart.yaml | yq -r '.version'`
image := "tinyops/pw"
nginxImage := `cat helm-chart/values.yaml | yq -r '.nginx.image.repository + ":" + .nginx.image.tag'`
redisImage := `cat helm-chart/values.yaml | yq -r '.redis.image.repository + ":" + .redis.image.tag'`
trivyReportFile := "docs/security/trivy-scan-report.txt"
dockleReportFile := "docs/security/dockle-scan-report.txt"

cleanup:
    rm -f {{ chartName }}-*.tgz

init: cleanup
    rustup component add clippy
    cargo install cargo-llvm-cov cargo-crev

bump-frontend-deps:
    cd frontend && yarn upgrade

bump-backend-deps:
    cargo update

bump-deps: bump-frontend-deps && bump-backend-deps

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

# DEV ENV

run-backend:
    cargo run

run-frontend:
    cd frontend && yarn && npm run dev -- --port=4200

start-dev-image:
    docker compose -f docker-compose-dev.yml up -d --build --force-recreate

stop-dev-image:
    docker compose -f docker-compose-dev.yml down

# HELM CHART
test-chart:
    helm template helm-chart/

build-chart: test-chart
    helm package helm-chart/ --app-version {{ version }}

release-chart: build-chart
    rm -rf helm-repo
    git clone git@github.com:tinyops-ru/tinyops-ru.github.io.git helm-repo
    bash -euo pipefail -c '\
        cd helm-repo && \
        cp ../{{ chartName }}-{{ chartVersion }}.tgz helm-charts/ && \
        helm repo index helm-charts/ && \
        if [ -z "$(git status --porcelain)" ]; then \
            echo "Chart {{ chartName }}-{{ chartVersion }} already published, skipping." && \
            exit 0; \
        fi && \
        git add helm-charts/ && \
        git commit -m "Add helm chart: {{ chartName }}-{{ chartVersion }}" && \
        git push'
    rm -rf helm-repo

# SECURITY
trivy:
    trivy image --severity HIGH,CRITICAL {{ image }}:{{ version }}

# RELEASE
build-release-image: lint && test
    docker build --progress=plain --platform=linux/amd64 -t {{ image }}:{{ version }} .

push-image:
    docker push {{ image }}:{{ version }}

trivy-save-reports:
    trivy -v > {{ trivyReportFile }}
    trivy config Dockerfile >> {{ trivyReportFile }}
    trivy image --severity HIGH,CRITICAL {{ image }}:{{ version }} >> {{ trivyReportFile }}
    echo "\n=== Redis Image Scan ===" >> {{ trivyReportFile }}
    trivy image --severity HIGH,CRITICAL {{ redisImage }} >> {{ trivyReportFile }}
    echo "\n=== Nginx Image Scan ===" >> {{ trivyReportFile }}
    trivy image --severity HIGH,CRITICAL {{ nginxImage }} >> {{ trivyReportFile }}

dockle-scan-reports:
    dockle --no-color {{ image }}:{{ version }} > {{ dockleReportFile }}

release-image: build-release-image
    @just push-image

release: build-release-image && release-chart
    @just push-image
    @just trivy-save-reports
    @just dockle-scan-reports
    echo "---"
    echo "Release completed successfully."
