FROM rust:1.53

WORKDIR /usr/src/myapp

COPY . .

RUN cp .env.example .env

RUN cp rocket.toml.example rocket.toml

RUN cp rocket.toml Rocket.toml

RUN cp diesel.toml Diesel.toml

RUN cargo install --path .

RUN apt update

RUN apt -y install postgresql

EXPOSE 8000

CMD ["./target/release/rust_backend"]