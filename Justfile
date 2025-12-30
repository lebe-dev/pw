version := `cat Cargo.toml | grep version | head -1 | cut -d " " -f 3 | tr -d "\""`
image := "tinyops/pw"

test-image-build:
  docker build --progress=plain -t app:dev .

run-backend:
  cargo run --bin server

run-frontend:
  cd frontend && yarn && npm run dev -- --port=4200

test-all:
  cd frontend && yarn test run
  cargo test --lib
  cargo test --bin server

build-release-image: test-all
  docker build --progress=plain --platform=linux/amd64 -t {{image}}:{{version}} .

build-chart:
  helm package helm-chart/

trivy:
  trivy image --severity HIGH,CRITICAL {{image}}:{{version}}

release: build-release-image
  docker push {{image}}:{{version}}
  helm package helm-chart/
