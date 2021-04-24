FROM ubuntu:latest

ENV TZ=America/Chicago
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# install build dependencies
RUN apt update -y && \
    apt install -y git make curl nodejs npm pandoc && \
    rm -rf /var/lib/apt/lists/*

# install zola
RUN curl -s -L https://github.com/getzola/zola/releases/download/v0.13.0/zola-v0.13.0-x86_64-unknown-linux-gnu.tar.gz \
    | tar xzf - -C /usr/local/bin

# install rust
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# copy entrypoint for github action
COPY action.sh /action.sh
ENTRYPOINT ["bash", "/action.sh"]
