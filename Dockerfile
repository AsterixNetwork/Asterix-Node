#FROM paritytech/ci-linux:production as builder
FROM alpine

LABEL description="This is the build stage for asterix. Here we create the binary."

ARG PROFILE=release
WORKDIR /asterix

COPY . /asterix/
#RUN  fallocate -l 1G /swapfile
RUN rustup uninstall nightly
RUN rustup install nightly-2021-03-01
RUN rustup update nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly-2021-03-01


RUN cargo build --$PROFILE -j 1

# ===== SECOND STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the asterix binary."
ARG PROFILE=release
COPY --from=builder /asterix/target/$PROFILE/asterix /usr/local/bin

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