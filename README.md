# helm-charts

Official Helm Chart Repository for Joxit's Applications. The Helm repository is located at `https://helm.joxit.dev`.

You can add the Helm repository to you environment with this command line. I will name the Helm repository `joxit` in all my charts.

```
helm repo add joxit https://helm.joxit.dev
```

[`helm-docs`](https://github.com/Joxit/helm-charts/tree/main/helm-docs) is a rust project that generate documentation for helm charts based on `values.yaml` file. It can also validate the app versions before a release.

Here is the list of all available charts.

## Docker Registry UI

[Docker Registry UI](https://github.com/Joxit/docker-registry-ui) aims to provide a simple and complete user interface for your private docker registry. You can customize the interface with various options. You can now easily install this project on a Kubernetes cluster thanks to `joxit/docker-registry-ui` Helm Chart.

Useful links for the installation:
- [The full documentation](https://helm.joxit.dev/charts/docker-registry-ui/)
- [Helm Chart source code on GitHub](https://github.com/Joxit/helm-charts/tree/main/charts/docker-registry-ui),
- [Default `values.yaml`](https://github.com/Joxit/helm-charts/tree/main/charts/docker-registry-ui/values.yaml).
- [Artifact HUB package](https://artifacthub.io/packages/helm/joxit/docker-registry-ui)
- [Docker Registry UI Chart changelog](https://github.com/Joxit/helm-charts/releases?q=docker-registry-ui&expanded=true)
- [Docker Registry UI Project changelog](https://github.com/Joxit/docker-registry-ui/releases)
