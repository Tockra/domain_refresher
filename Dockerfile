FROM rust:latest

COPY ./* /home/refresher/
WORKDIR /home/refresher/

RUN cargo build --release
RUN mv 

CMD ["/home/refresher/target/release/domain_refresher"]