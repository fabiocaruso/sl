FROM arm64v8/ubuntu
WORKDIR /root/
EXPOSE 8000
VOLUME /root/downloads
COPY . .

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install curl build-essential ffmpeg libavcodec-extra python3 python-is-python3 npm libevent-dev llvm libclang-dev -y

RUN curl -L --output youtube-dl https://youtube-dl.org/downloads/latest/youtube-dl
RUN chmod +x youtube-dl
RUN mv youtube-dl /usr/local/bin/youtube-dl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo build --release
RUN cp target/release/soundloop /usr/local/bin/soundloop

ENTRYPOINT soundloop -d "/root/downloads" -w $SOUNDLOOP_WORKERS --dbhost $SOUNDLOOP_DBHOST --dbuser $SOUNDLOOP_DBUSER --jwt $SOUNDLOOP_JWT --dbpwd $SOUNDLOOP_DBPW
