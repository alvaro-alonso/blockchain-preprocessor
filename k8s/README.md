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
    $ minikube kubectl --  -n st-alonso get ns,svc,deploy
    ```
    If ns, svc, and deploy not setup:
    ```sh
    $ minikube kubectl -- create -f namespace.yaml -f prover.yaml
    ```
    
1. Open service:
    ```
    minikube service -n st-alonso zokrates-service --url
    ```
    Click on the output link and go to the `/docs` url. You should now see the OpenAPI interface on your browser.

