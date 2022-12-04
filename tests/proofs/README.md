## Run Zokrates

- start the zokrates container and the proofs in this directory with:
  `$ docker run -it -v "$(pwd)"/src:/home/zokrates/proofs/src -v "$(pwd)"/lib:/home/zokrates/proofs/lib zokrates/zokrates:0.8.3` 