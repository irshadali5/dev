---
title: "Expert Architecture: Kubernetes Helm Charts"
description: "For cloud-agnostic deployments, we use Kubernetes. Below is our production-ready Helm chart `values.yaml` for deploying the API, Postgres, Redis, and OpenTelemetry."
order: 28
---

For cloud-agnostic deployments, we use Kubernetes. Below is our production-ready Helm chart \`values.yaml\` for deploying the API, Postgres, Redis, and OpenTelemetry.

**`helm/zero2prod/values.yaml`**

```yaml
replicaCount: 3

image:
  repository: my-registry.com/zero2prod
  pullPolicy: IfNotPresent
  tag: "1.0.0"

service:
  type: ClusterIP
  port: 80

ingress:
  enabled: true
  className: "nginx"
  annotations:
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
  hosts:
    - host: api.example.com
      paths:
        - path: /
          pathType: Prefix

resources:
  limits:
    cpu: 1000m
    memory: 512Mi
  requests:
    cpu: 100m
    memory: 128Mi

autoscaling:
  enabled: true
  minReplicas: 3
  maxReplicas: 20
  targetCPUUtilizationPercentage: 80

redis:
  enabled: true
  architecture: replication
  auth:
    enabled: true

postgresql:
  enabled: true
  architecture: standalone
      
```