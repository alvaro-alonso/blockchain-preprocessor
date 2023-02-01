
# Generation of a valid proof for testing purposes

## Example Eddsa proof in zokrates

1. Build the image and start a container from the `Dockerfile`
1. In the container run following commands:
    ```sh
    # compile the .zok file 
    $ zokrates compile -i proof_of_ownership.zok -o proof_of_ownership

    # run the setup with ark as backend and the gm17 proving-scheme
    $ zokrates setup -b ark -s gm17 -i proof_of_ownership

    # generate a witness from a valid proof
    $ zokrates compute-witness -i proof_of_ownership -a 10041775272610680597649138558111867140088287599035431170728241228669634925671 19045584355489137154300255038437027652180257880634202059955435891798466344432 14517916597883362893064608394843629693674165114908520112595055382047085957383 14897476871502190904409029696666322856887678969656209656241038339251270171395 16668832459046858928951622951481252834155254151733002984053501254009901876174 3814687126 4207057211 2301474087 1696421512 1054042432 4114589074 2402006685 2358319779 2636307903 771130895 3338794104 910337493 3941248527 2566242658 3403499691 2178970740

    # or generate a witness from a a json file
    $ zokrates compute-witness --abi -i proof_of_ownership --stdin < witness_abi.json

    # generate the proof with ark as backend and the gm17 proving-scheme
    $ zokrates generate-proof -b ark -s gm17 -i proof_of_ownership
    ```
1. Copy the necessary files to the host machine:
    ```sh
    $ docker cp <CONTAINER-ID>:/home/zokrates/proof.json .
    $ docker cp <CONTAINER-ID>:/home/zokrates/witness .
    $ docker cp <CONTAINER-ID>:/home/zokrates/proving.key .
    $ docker cp <CONTAINER-ID>:/home/zokrates/proof_of_ownership .
    ```

## How create an eddsa key pair and signature

To generate the eddsa keys and signature [pycrypto](https://github.com/Zokrates/pycrypto#create-and-verify-an-eddsa-signature)'s example is used.

To generate a valid witness for a certain key pair use `demo.py` template in [pycrypto](https://github.com/Zokrates/pycrypto#create-and-verify-eddsa-signature) und run the script in pycrytos repository


