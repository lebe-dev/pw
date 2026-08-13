# Load variables from .env (gitignored) into recipe environments — e.g. SONAR_TOKEN.
set dotenv-load

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

# Backend coverage -> lcov.info, with SF: paths relative to the workspace root.
test-backend-coverage:
    #!/usr/bin/env bash
    set -euo pipefail
    # --remap-path-prefix is required: without it llvm-cov writes absolute host paths,
    # which the scanner container (repo mounted at /usr/src) cannot resolve.
    cargo llvm-cov --lcov --remap-path-prefix --output-path lcov.info

# Frontend coverage -> frontend/coverage/lcov.info, with SF: paths repo-root relative.
test-frontend-coverage:
    #!/usr/bin/env bash
    set -euo pipefail
    cd frontend && yarn vitest run --coverage
    # Vitest writes SF: paths relative to frontend/, but the scanner resolves them from
    # the repo root. Plain sed instead of `sed -i`, whose syntax differs between BSD and
    # GNU; the redirect (not mktemp) keeps the report world-readable for the scanner
    # container, which runs as a different uid.
    report="coverage/lcov.info"
    sed 's|^SF:|SF:frontend/|' "$report" > "$report.tmp"
    mv "$report.tmp" "$report"

# Both coverage reports, as consumed by sonar-project.properties.
test-coverage: test-backend-coverage test-frontend-coverage

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

# --- SonarQube (static analysis) ---
# Host URL as seen from *inside* the scanner container. host.docker.internal
# reaches the host's published port 9000 on Docker Desktop and (via --add-host)
# on Linux. Override with SONAR_HOST_URL when scanning a remote instance.
sonarHostUrl := env_var_or_default("SONAR_HOST_URL", "http://host.docker.internal:9000")

sonar-scan: test-backend-coverage test-frontend-coverage
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -z "${SONAR_TOKEN:-}" ]; then
        echo "error: SONAR_TOKEN is not set." >&2
        echo "  Generate a token at {{ sonarHostUrl }} -> My Account -> Security," >&2
        echo "  then add it to .env:  SONAR_TOKEN=sqp_xxx" >&2
        exit 1
    fi
    docker run --rm \
        --add-host=host.docker.internal:host-gateway \
        -e SONAR_HOST_URL="{{ sonarHostUrl }}" \
        -e SONAR_TOKEN="$SONAR_TOKEN" \
        -v "$PWD:/usr/src" \
        sonarsource/sonar-scanner-cli:latest \
        -Dsonar.projectVersion="{{ version }}"

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
