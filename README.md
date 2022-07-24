# helm-charts

Official Helm Chart Repository for Joxit Applications


## Prerequisites
  * **Helm 3.2+** (Helm 2 is not supported)
  * **Kubernetes 1.19+** - This is the earliest version of Kubernetes tested.
    It is possible that this chart works with earlier versions but it is
    untested.

## Usage

1. Add my Helm repository (named `joxit`)
```
helm repo add joxit https://helm.joxit.dev
```
2. Ensure you have access to the Helm chart and you see the latest chart version listed. If you have previously added the Helm repository, run `helm repo update`.
```
helm search repo joxit/docker-registry-ui
```

## Docker Registry UI

[Docker Registry UI](https://github.com/Joxit/docker-registry-ui) aims to provide a simple and complete user interface for your private docker registry. You can customize the interface with various options. You can now install this project on a Kubernetes cluster thanks to this Helm Chart. Checkout the [Helm source code](https://github.com/Joxit/helm-charts/tree/main/charts/docker-registry-ui), [the full documentation](https://helm.joxit.dev/charts/docker-registry-ui/) and the [default values.yaml](https://github.com/Joxit/helm-charts/tree/main/charts/docker-registry-ui/values.yaml).
