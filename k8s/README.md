## Minikube

1. Start minikube
    ```sh
    $ minikube start
    $ minikube kubectl --  -n st-alonso get ns,svc,deploy
    ```
    If ns, svc, and deploy not setup:
    ```sh
    $ minikube kubectl -- create -f namespace.yaml -f prover.yaml
    ```

1. Expose service
    ```sh
    minikube service -n st-alonso zokrates-service --url
    ```
    Click on the output link and go to the `/docs` url. You should now see the OpenAPI interface on your browser.
