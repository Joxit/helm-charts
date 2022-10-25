# Docker Registry UI Chart

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
| `ui.title` | `Docker registry UI` | Title of the registry |
| `ui.deleteImages` | `false` | Allow delete of images |
| `ui.proxy` | `false` | UI behave as a proxy of the registry |
| `ui.dockerRegistryUrl` | `null` | The URL of your docker registry, may be a service (when proxy is on) or an external URL. |
| `ui.pullUrl` | `null` | Override the pull URL |
| `ui.showCatalogNbTags` | `false` | Show number of tags per images on catalog page. This will produce + nb images requests, not recommended on large registries. |
| `ui.showContentDigest` | `false` | Show content digest in docker tag list. |
| `ui.singleRegistry` | `true` | Remove the menu that show the dialogs to add, remove and change the endpoint of your docker registry. |
| `ui.catalogElementsLimit` | `100000` | Limit the number of elements in the catalog page. |
| `ui.historyCustomLabels` | `[]` | Expose custom labels in history page, custom labels will be processed like maintainer label. |
| `ui.nginxProxyHeaders` | `[]` | Update the default Nginx configuration and **set custom headers** for your backend docker registry. Only when `ui.proxy` is used. |
| `ui.nginxProxyPassHeaders` | `[]` | Update the default Nginx configuration and **forward custom headers** to your backend docker registry. Only when `ui.proxy` is used. |
| `ui.useControlCacheHeader` | `false` | Add header `Control-Cache: no-store, no-cache` on requests to registry server. |
| `ui.runAsRoot` | `true` | Use root or nginx user inside the container, when this is false the target port must be greater or equal to 1024. |
| `ui.image` | `joxit/docker-registry-ui:2.3.0` | The name and tag of the docker image of the interface |
| `ui.imagePullSecrets` | `-` | Override default image pull secrets |
| `ui.imagePullPolicy` | `-` | Override default pull policy |
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
| `ui.ingress.enable` | `false` | Enable the ingress for the user interface. |
| `ui.ingress.host` | `null` | Fully qualified domain name of a network host. |
| `ui.ingress.path` | `/` | Path is matched against the path of an incoming request. |
| `ui.ingress.pathType` | `Prefix` | Determines the interpretation of the Path matching, must be Prefix to serve assets. |
| `ui.ingress.ingressClassName` | `nginx` | The name of an IngressClass cluster resource. |
| `ui.ingress.tls` | `[]` | Optional YAML tls configuration. |
| `ui.ingress.annotations` | `{}` | Annotations to apply to the user interface ingress. |

### Registry Server

| Value | Default | Description |
| --- | --- | --- |
| `registry.enabled` | `false` | Enable the registry server. |
| `registry.image` | `registry:2.8.1` | The name and tag of the docker registry server image |
| `registry.imagePullSecrets` | `-` | Override default image pull secrets |
| `registry.imagePullPolicy` | `-` | Override default pull policy |
| `registry.dataVolume` | `null` | Configuration for the data directory. When null it will create an emptyDir. |
| `registry.resources` | `{}` | The resource settings for registry server pod. |
| `registry.nodeSelector` | `{}` | Optional YAML string to specify a nodeSelector config. |
| `registry.tolerations` | `[]` | Optional YAML string to specify tolerations. |
| `registry.extraEnv` | `[]` | Extra Environmental Variables for Registry. |
| `registry.affinity` | `{}` | This value defines the [affinity](https://kubernetes.io/docs/concepts/configuration/assign-pod-node/#affinity-and-anti-affinity) for server pods. |
| `registry.annotations` | `{}` | Annotations to apply to the registry server deployment. |
| `registry.additionalSpec` | `{}` | Optional YAML string that will be appended to the deployment spec. |
| `registry.service.type` | `ClusterIP` | Type of service: `LoadBalancer`, `ClusterIP` or `NodePort`. If using `NodePort` service type, you must set the desired `nodePorts` setting below. |
| `registry.service.port` | `5000` | Ports that will be exposed on the service |
| `registry.service.targetPort` | `5000` | The port to listhen on the container. |
| `registry.service.nodePort` | `null` | If using a `NodePort` service type, you must specify the desired `nodePort` for each exposed port. |
| `registry.service.annotations` | `{}` | Annotations to apply to the user interface service. |
| `registry.service.additionalSpec` | `{}` | Optional YAML string that will be appended to the Service spec. |
| `registry.ingress.enable` | `false` | Enable the ingress for the registry server. |
| `registry.ingress.host` | `null` | Fully qualified domain name of a network host. |
| `registry.ingress.path` | `/v2/` | Path is matched against the path of an incoming request. |
| `registry.ingress.pathType` | `Prefix` | Determines the interpretation of the Path matching, must be Prefix to serve assets. |
| `registry.ingress.ingressClassName` | `nginx` | The name of an IngressClass cluster resource. |
| `registry.ingress.tls` | `[]` | Optional YAML tls configuration. |
| `registry.ingress.annotations` | `{}` | Annotations to apply to the registry server ingress. |
