FROM rust:1.66.0-bullseye
ENV OUT_FILE=output.log
RUN echo "deb [signed-by=/usr/share/keyrings/cloud.google.gpg] http://packages.cloud.google.com/apt cloud-sdk main" | tee -a /etc/apt/sources.list.d/google-cloud-sdk.list
RUN curl https://packages.cloud.google.com/apt/doc/apt-key.gpg |  apt-key --keyring /usr/share/keyrings/cloud.google.gpg add -
RUN apt-get update
RUN apt-get install -y git build-essential pkg-config libssl-dev
RUN git clone https://github.com/delendum-xyz/zk-benchmarking.git
RUN apt-get install google-cloud-sdk
CMD "/bin/bash"