#Deriving the latest base image
FROM python:3.9.7-slim

WORKDIR /usr/app/src
# RUN mkdir ../results

COPY . ./

RUN pip install -r requirements.txt

# CMD [ "python", "./run_test.py"]