
# Generation of a valid proof for testing purposes

## Example Eddsa proof in zokrates

1. Build the image and start a container from the `Dockerfile`
1. In the container run following commands:
    ```sh
    # compile the .zok file 
    $ zokrates compile -i x_signatures.zok

    # run the setup with ark as backend and the gm17 proving-scheme
    $ zokrates setup -b ark -s gm17

    # or generate a witness from a a json file
    $ zokrates compute-witness --abi -i proof_of_ownership --stdin < witness_abi.json

    # generate the proof with ark as backend and the gm17 proving-scheme
    $ zokrates generate-proof -b ark -s gm17 -i proof_of_ownership
    ```

## How create an eddsa key pair and signature

To generate the eddsa keys and signature [pycrypto](https://github.com/Zokrates/pycrypto#create-and-verify-an-eddsa-signature)'s example is used.

To generate a valid witness for a certain key pair use `demo.py` template in [pycrypto](https://github.com/Zokrates/pycrypto#create-and-verify-eddsa-signature) und run the script in pycrytos repository


