#Deriving the latest base image
FROM python:3.9.7-slim

WORKDIR /usr/app/src


COPY . ./

RUN mkdir /usr/app/results

ENV CLUSTER_ADDRESS=http://zokrates-service:8000

RUN pip install -r requirements.txt

# CMD [ "python", "./run_test.py"]