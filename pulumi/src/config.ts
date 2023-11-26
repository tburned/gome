import { InfraConfig } from "./types";

import * as pulumi from "@pulumi/pulumi";

const createConfig = (): InfraConfig => {
  const config = new pulumi.Config();

  return {
    isMinikube: config.requireBoolean("isMinikube"),
    k8sNamespace: config.get("namespace") || "default",
    numRepliaces: config.getNumber("replicas") || 1,
    stack: pulumi.getStack(),
  };
};

const infraConfig = createConfig();
export default infraConfig;
