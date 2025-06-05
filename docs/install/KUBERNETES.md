# How to install PW to Kubernetes

```bash
helm upgrade --install --create-namespace -n pw pw oci://registry-1.docker.com/tinyops/pw --version 0.1.0 --values values.yaml
```
