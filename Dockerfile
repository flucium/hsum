FROM ubuntu:22.04

RUN apt update && apt upgrade -y && \
apt install -y build-essential curl wget vim unzip && \
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
mkdir ./repos && cd ./repos && \
wget https://github.com/flucium/hsum/archive/refs/heads/main.zip && \
unzip main.zip && mv hsum-main hsum && rm main.zip && cd hsum && \
cargo build --debug

ENV PATH="/root/.cargo/bin:$PATH"

CMD /bin/bash