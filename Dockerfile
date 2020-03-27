FROM rust:alpine

COPY ./ /home/refresher/
WORKDIR /home/refresher/

RUN cargo build --release

ENV TZ=Europe/Berlin
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

CMD ["/home/refresher/target/release/domain_refresher"]
