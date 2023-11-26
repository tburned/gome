import * as k8s from "@pulumi/kubernetes";
import * as docker from "@pulumi/docker";
import { InfraConfig } from "./types";
import { Output } from "@pulumi/pulumi";

export type DockerRegistry = {
  deployment: k8s.apps.v1.Deployment;
  ip: Output<string>;
};

const deployDockerRegistry: (config: InfraConfig) => DockerRegistry = (
  config: InfraConfig
) => {
  const dockerRegistryName = "registry";
  const dockerRegistryImage = new docker.RemoteImage(
    `${dockerRegistryName}Image`,
    {
      name: "registry:2.7",
    }
  );

  const appName = "docker-registry";
  const appLabels = { app: appName };
  const deployment = new k8s.apps.v1.Deployment(appName, {
    spec: {
      selector: { matchLabels: appLabels },
      replicas: 1,
      template: {
        metadata: { labels: appLabels },
        spec: {
          containers: [
            { name: dockerRegistryName, image: dockerRegistryImage.imageId },
          ],
        },
      },
    },
  });

  // Allocate an IP to the Deployment.
  const registry = new k8s.core.v1.Service(appName, {
    metadata: { labels: deployment.spec.template.metadata.labels },
    spec: {
      type: config.isMinikube ? "ClusterIP" : "LoadBalancer",
      ports: [{ port: 5000, targetPort: 5000, protocol: "TCP" }],
      selector: appLabels,
    },
  });

  // When "done", this will print the public IP.
  const ip = config.isMinikube
    ? registry.spec.clusterIP
    : registry.status.loadBalancer.apply(
        (lb) => lb.ingress[0].ip || lb.ingress[0].hostname
      );

  return {
    deployment,
    ip,
  };
};

export default deployDockerRegistry;
