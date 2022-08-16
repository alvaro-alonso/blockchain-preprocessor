# Setup 

## Docker
If you have Docker installed in your machine you can simply run:
```bash
$ docker-compose up
```

# Contribute

## pre-commit hooks
In order to maintain the linting and style guidelines please install `pre-commit` hook by running the following:

```sh
$ chmod +x hooks/pre-commit
$ cd .git/hooks
$ ln -s -f ../../hooks/pre-commit ./pre-commit  
```