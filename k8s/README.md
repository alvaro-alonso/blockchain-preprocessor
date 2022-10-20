# Deploy Cluster locally

1. Start minikube:
    ```
    $ minikube start
    $ minikube profile list
    ```
1. Pull Docker image:
    ```
    $ minikube ssh docker pull alvaround/zokrates-api:latest
    ```
1. Create deploy and service:
    ```
    $ kubectl create -f prover.yaml
    $ kubectl get all -l app=prover-node
    ```
1. Open service:
    ```
    $ minikube service prover-service
    ```
1. Open the follwing link in the browser: http://127.0.0.1:56114/docs