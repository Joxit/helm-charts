# Docker Registry UI Chart

[![Stars](https://img.shields.io/github/stars/joxit/docker-registry-ui.svg?logo=github&maxAge=86400)](https://github.com/Joxit/docker-registry-ui/stargazers)
[![Pulls](https://img.shields.io/docker/pulls/joxit/docker-registry-ui.svg?maxAge=86400)](https://hub.docker.com/r/joxit/docker-registry-ui)
[![Sponsor](https://joxit.dev/images/sponsor.svg)](https://github.com/sponsors/Joxit)
[![Artifact Hub](https://img.shields.io/endpoint?url=https://artifacthub.io/badge/repository/joxit)](https://artifacthub.io/packages/helm/joxit/docker-registry-ui)

## Overview

This project aims to provide a simple and complete user interface for your private docker registry. You can customize the interface with various options. The major option is `ui.singleRegistry` which allows you to disable the dynamic selection of docker registeries.

If you like my work and want to support it, don't hesitate to [sponsor me](https://github.com/sponsors/Joxit).

## [Project Page](https://joxit.dev/docker-registry-ui), [Live Demo](https://joxit.dev/docker-registry-ui/demo/), [Examples](https://github.com/Joxit/docker-registry-ui/tree/main/examples), [Helm Chart](https://helm.joxit.dev/charts/docker-registry-ui/)

![preview](https://raw.github.com/Joxit/docker-registry-ui/main/docker-registry-ui.gif "Preview of Docker Registry UI")

## Prerequisites

  * **Helm 3.2+** (Helm 2 is not supported)
  * **Kubernetes 1.19+** - This is the earliest version of Kubernetes tested.
    It is possible that this chart works with earlier versions but it is untested.

## Usage

1. Add my Helm repository (named `joxit`)
```
helm repo add joxit https://helm.joxit.dev
```
2. Ensure you have access to the Helm chart and you see the latest chart version listed. If you have previously added the Helm repository, run `helm repo update`.
```
helm search repo joxit/docker-registry-ui
```
3. Now you're ready to install the Docker Registry UI! To install Docker Registry UI with the default configuration using Helm 3.2 run the following command below. This will deploy the Docker Registry UI on the default namespace.
```
helm upgrade --install docker-registry-ui joxit/docker-registry-ui
```

## Configuration

### Global

| Value | Default | Description |
| --- | --- | --- |
| `global.name` | `null` | Set the prefix used for all resources in the Helm chart. If not set, the prefix will be `<helm release name>`. |
| `global.imagePullSecrets` | `[]` | The default array of objects containing image pull secret names that will be applied. |
| `global.imagePullPolicy` | `IfNotPresent` | The default image policy for images: `IfNotPresent`, `Always`, `Never` |

### User Interface

| Value | Default | Description |
| --- | --- | --- |
| `ui.replicas` | `1` | Number of replicas for the Deployment. |
| `ui.title` | `"Docker registry UI"` | Title of the registry |
| `ui.proxy` | `false` | UI behave as a proxy of the registry |
| `ui.dockerRegistryUrl` | `null` | The URL of your docker registry, may be a service (when proxy is on) or an external URL. |
| `ui.pullUrl` | `null` | Override the pull URL |
| `ui.singleRegistry` | `true` | Remove the menu that show the dialogs to add, remove and change the endpoint of your docker registry. |
| `ui.registrySecured` | `false` | By default, the UI will check on every requests if your registry is secured or not (you will see `401` responses in your console). Set to `true` if your registry uses Basic Authentication and divide by two the number of call to your registry. |
| `ui.showCatalogNbTags` | `false` | Show number of tags per images on catalog page. This will produce + nb images requests, not recommended on large registries. |
| `ui.catalogElementsLimit` | `1000` | Limit the number of elements in the catalog page. |
| `ui.catalogDefaultExpanded` | `false` | Expand by default all repositories in catalog |
| `ui.catalogMinBranches` | `1` | Set the minimum repository/namespace to expand (e.g. `joxit/docker-registry-ui` `joxit/` is the repository/namespace). Can be 0 to disable branching. |
| `ui.catalogMaxBranches` | `1` | Set the maximum repository/namespace to expand (e.g. `joxit/docker-registry-ui` `joxit/` is the repository/namespace). Can be 0 to disable branching. |
| `ui.deleteImages` | `false` | Allow delete of images |
| `ui.showContentDigest` | `false` | Show content digest in docker tag list. |
| `ui.taglistOrder` | `alpha-asc;num-desc` | Set the default order for the taglist page, could be `num-asc;alpha-asc`, `num-desc;alpha-asc`, `num-asc;alpha-desc`, `num-desc;alpha-desc`, `alpha-asc;num-asc`, `alpha-asc;num-desc`, `alpha-desc;num-asc` or `alpha-desc;num-desc`. |
| `ui.taglistPageSize` | `100` | Set the number of tags to display in one page. |
| `ui.historyCustomLabels` | `[]` | Expose custom labels in history page, custom labels will be processed like maintainer label. |
| `ui.nginxProxyHeaders` | `[]` | Update the default Nginx configuration and **set custom headers** for your backend docker registry. Only when `ui.proxy` is used. Example: nginxProxyHeaders:  [ { my-heeader-name: my-header-value } ] |
| `ui.nginxProxyPassHeaders` | `[]` | Update the default Nginx configuration and **forward custom headers** to your backend docker registry. Only when `ui.proxy` is used. Example: nginxProxyPassHeaders: [ my-first-header, my-second-header ] |
| `ui.useControlCacheHeader` | `false` | Add header Control-Cache: no-store, no-cache on requests to registry server. This needs to update your registry configuration with : `Access-Control-Allow-Headers: ['Authorization', 'Accept', 'Cache-Control']` |
| `ui.runAsRoot` | `true` | Use root or nginx user inside the container, when this is false the target port must be greater or equal to 1024. |
| `ui.defaultTheme` | `"auto"` | Select the default theme to apply, values can be `auto`, `dark` and `light` |
| `ui.theme.background` | `""` | Custom background color for the UI |
| `ui.theme.primaryText` | `""` | Custom primary text color for the UI |
| `ui.theme.neutralText` | `""` | Custom netral color for the UI (icons) |
| `ui.theme.accentText` | `""` | Custom accent color for the UI (buttons) |
| `ui.theme.hoverBackground` | `""` | Custom hover background color for the UI |
| `ui.theme.headerBackground` | `""` | Custom header background color for the UI |
| `ui.theme.headerText` | `""` | Custom header text color for the UI |
| `ui.theme.footerBackground` | `""` | Custom footer background color for the UI |
| `ui.theme.footerText` | `""` | Custom footer text color for the UI |
| `ui.theme.footerNeutralText` | `""` | Custom footer neutral color for the UI (links) |
| `ui.image` | `joxit/docker-registry-ui:2.5.2` | The name and tag of the docker image of the interface |
| `ui.imagePullSecrets` | `"-"` | Override default image pull secrets |
| `ui.imagePullPolicy` | `"-"` | Override default pull policy |
| `ui.resources` | `{}` | The resource settings for user interface pod. |
| `ui.nodeSelector` | `{}` | Optional YAML string to specify a nodeSelector config. |
| `ui.tolerations` | `[]` | Optional YAML string to specify tolerations. |
| `ui.affinity` | `{}` | This value defines the [affinity](https://kubernetes.io/docs/concepts/configuration/assign-pod-node/#affinity-and-anti-affinity) for server pods. |
| `ui.annotations` | `{}` | Annotations to apply to the user interface deployment. |
| `ui.additionalSpec` | `{}` | Optional YAML string that will be appended to the deployment spec. |
| `ui.service.type` | `ClusterIP` | Type of service: `LoadBalancer`, `ClusterIP` or `NodePort`. If using `NodePort` service type, you must set the desired `nodePorts` setting below. |
| `ui.service.port` | `80` | Ports that will be exposed on the service |
| `ui.service.targetPort` | `80` | The port to listhen on the container. If under 1024, the user must be root |
| `ui.service.nodePort` | `null` | If using a `NodePort` service type, you must specify the desired `nodePort` for each exposed port. |
| `ui.service.annotations` | `{}` | Annotations to apply to the user interface service. |
| `ui.service.additionalSpec` | `{}` | Optional YAML string that will be appended to the Service spec. |
| `ui.ingress.enabled` | `false` | Enable the ingress for the user interface. |
| `ui.ingress.host` | `null` | Fully qualified domain name of a network host. |
| `ui.ingress.path` | `/` | Path is matched against the path of an incoming request. |
| `ui.ingress.pathType` | `Prefix` | Determines the interpretation of the Path matching, must be Prefix to serve assets. |
| `ui.ingress.ingressClassName` | `nginx` | The name of an IngressClass cluster resource. |
| `ui.ingress.tls` | `[]` | TLS configuration |
| `ui.ingress.annotations` | `{}` | Annotations to apply to the user interface ingress. |

### Registry Server

| Value | Default | Description |
| --- | --- | --- |
| `registry.enabled` | `false` | Enable the registry server. |
| `registry.image` | `registry:2.8.2` | The name and tag of the docker registry server image |
| `registry.imagePullSecrets` | `"-"` | Override default image pull secrets |
| `registry.imagePullPolicy` | `"-"` | Override default pull policy |
| `registry.dataVolume` | `null` | Configuration for the data directory.  When null it will create an emptyDir. |
| `registry.resources` | `{}` | The resource settings for registry server pod. |
| `registry.nodeSelector` | `{}` | Optional YAML string to specify a nodeSelector config. |
| `registry.tolerations` | `[]` | Optional YAML string to specify tolerations. |
| `registry.affinity` | `{}` | This value defines the [affinity](https://kubernetes.io/docs/concepts/configuration/assign-pod-node/#affinity-and-anti-affinity) for server pods. |
| `registry.annotations` | `{}` | Annotations to apply to the registry server deployment. |
| `registry.additionalSpec` | `{}` | Optional YAML string that will be appended to the deployment spec. |
| `registry.extraEnv` | `[]` | Extra Environmental Variables for Registry |
| `registry.auth.basic.enabled` | `false` | Enable basic auth for Registry. |
| `registry.auth.basic.realm` | `Docker registry` | Basic auth realm. |
| `registry.auth.basic.htpasswdPath` | `/etc/docker/registry/auth/htpasswd` | Full path for htpasswd file. Note that filename should match the secret key. |
| `registry.auth.basic.secretName` | `''` | htpasswd secret name volume to mount. |
| `registry.service.type` | `ClusterIP` | Type of service: `LoadBalancer`, `ClusterIP` or `NodePort`. If using `NodePort` service type, you must set the desired `nodePorts` setting below. |
| `registry.service.port` | `5000` | Ports that will be exposed on the service |
| `registry.service.targetPort` | `5000` | The port to listhen on the container. |
| `registry.service.nodePort` | `null` | If using a `NodePort` service type, you must specify the desired `nodePort` for each exposed port. |
| `registry.service.annotations` | `{}` | Annotations to apply to the registry server service. |
| `registry.service.additionalSpec` | `{}` | Optional YAML string that will be appended to the Service spec. |
| `registry.ingress.enabled` | `false` | Enable the ingress for the registry server. |
| `registry.ingress.host` | `null` | Fully qualified domain name of a network host. |
| `registry.ingress.path` | `/v2/` | Path is matched against the path of an incoming request. |
| `registry.ingress.pathType` | `Prefix` | Determines the interpretation of the Path matching, must be Prefix to serve assets. |
| `registry.ingress.ingressClassName` | `nginx` | The name of an IngressClass cluster resource. |
| `registry.ingress.tls` | `[]` | TLS configuration |
| `registry.ingress.annotations` | `{}` | Annotations to apply to the registry server ingress. |
