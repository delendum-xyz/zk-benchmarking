FROM polymorpher/delendum-zk-benchmarking:v0.11-linux-x64
RUN apt-get update
RUN apt-get install awscli -y
WORKDIR /
COPY wrapper.sh /wrapper.sh
CMD "/wrapper.sh"