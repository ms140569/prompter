FROM debian:buster
RUN apt-get update
RUN apt install -y gcc
RUN apt install -y gdb
RUN apt install -y git
RUN apt install -y curl
RUN apt install -y procps
RUN curl https://sh.rustup.rs -sSf >/tmp/rustup.sh
RUN /bin/sh /tmp/rustup.sh -y
ENV PATH="/root/.cargo/bin:${PATH}"
ADD . /root/prompter

