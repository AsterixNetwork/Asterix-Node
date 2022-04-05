FROM paritytech/ci-linux:production as builder

LABEL description="This is the build stage for asterix-node. Here we create the binary."

ARG PROFILE=release
WORKDIR /asterix-node

COPY . /asterix-node/
#RUN  fallocate -l 1G /swapfile
RUN rustup install nightly-2021-03-01
RUN rustup target add wasm32-unknown-unknown --toolchain nightly-2021-03-01


RUN cargo +nightly-2021-03-01 build --$PROFILE 

# ===== SECOND STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the asterix-node binary."
ARG PROFILE=release
COPY --from=builder /asterix-node/target/$PROFILE/asterix /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /asterix asterix && \
	mkdir -p /asterix/.local/share && \
	mkdir /data && \
	chown -R asterix:asterix /data && \
	ln -s /data /asterix/.local/share/asterix && \
	rm -rf /usr/bin /usr/sbin

USER asterix
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/asterix"]