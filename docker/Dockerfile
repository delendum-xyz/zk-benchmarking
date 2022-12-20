FROM polymorpher/delendum-zk-benchmarking:v0.11-linux-x64
ENV BENCH_OUTPUT_FILE=output.log
ENV GCP_SERVICE_ACCOUNT_JSON_KEY=''
ENV GCP_PROJECT=''
ENV GCP_SERVICE_ACCOUNT=''
ENV GCP_BUCKET=''
WORKDIR /
COPY wrapper.sh /wrapper.sh
CMD "/wrapper.sh"