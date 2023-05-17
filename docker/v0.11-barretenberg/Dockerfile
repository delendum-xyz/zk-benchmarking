FROM barretenberg/external_benchmarks
RUN cd /usr/src/barretenberg/cpp/srs_db/ && ./download_ignition.sh 3 && cd ../build
RUN apk add --update python3 curl which bash
RUN curl -sSL https://sdk.cloud.google.com | bash
WORKDIR /usr/src/barretenberg/cpp/build
COPY benchmark_parser.sh ./benchmark_parser.sh
RUN chmod +x ./bin/external_bench
RUN chmod +x ./benchmark_parser.sh
CMD "/bin/bash"
