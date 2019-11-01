FROM python:3.6

ENV MLFLOW_VERSION 1.3.0
RUN pip install mlflow==$MLFLOW_VERSION

RUN mkdir -p /mlflow/

WORKDIR /mlflow/

EXPOSE 5000

CMD mlflow server --host 0.0.0.0
