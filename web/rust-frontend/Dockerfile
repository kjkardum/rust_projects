FROM rust:1.53

WORKDIR /usr/src/myapp

COPY . .

SHELL ["/bin/bash", "-c"]

RUN cargo install --locked trunk

RUN cargo install wasm-bindgen-cli

RUN rustup target add wasm32-unknown-unknown

RUN trunk build --release

EXPOSE 8080

CMD "trunk serve --release"